//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use async_std::{
    net::{SocketAddr, TcpListener, TcpStream},
    prelude::*,
    sync::Arc,
    task,
};
use clap::Parser;
use std::{
    convert::{TryFrom, TryInto},
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use zenoh_buffers::{
    reader::HasReader,
    writer::{HasWriter, Writer},
};
use zenoh_codec::{RCodec, WCodec, Zenoh060};
use zenoh_protocol::{
    core::{WhatAmI, ZenohId},
    transport::{InitSyn, OpenSyn, TransportBody, TransportMessage},
};

macro_rules! zsend {
    ($msg:expr, $stream:expr) => {{
        // Create the buffer for serializing the message
        let mut buff = Vec::new();
        let mut writer = buff.writer();
        let codec = Zenoh060::default();

        // Reserve 16 bits to write the length
        writer
            .write_exact(u16::MIN.to_le_bytes().as_slice())
            .unwrap();

        // Serialize the message
        codec.write(&mut writer, $msg).unwrap();

        // Write the length
        let num = u16::MIN.to_le_bytes().len();
        let len = u16::try_from(writer.len() - num).unwrap();
        buff[..num].copy_from_slice(len.to_le_bytes().as_slice());

        // Send the message on the link
        $stream.write_all(&buff).await.unwrap();
    }};
}

macro_rules! zrecv {
    ($stream:expr, $buffer:expr) => {{
        let _ = $stream.read_exact(&mut $buffer[0..2]).await.unwrap();
        let length: [u8; 2] = $buffer[0..2].try_into().unwrap();
        // Decode the total amount of bytes that we are expected to read
        let to_read = u16::from_le_bytes(length) as usize;
        $stream.read_exact(&mut $buffer[0..to_read]).await.unwrap();

        let mut reader = $buffer.reader();
        let codec = Zenoh060::default();
        let msg: TransportMessage = codec.read(&mut reader).unwrap();
        msg
    }};
}

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let my_whatami = WhatAmI::Router;
    let my_pid = ZenohId::rand();

    // Create the reading buffer
    let mut buffer = vec![0u8; 16_000_000];

    // Read the InitSyn
    let message = zrecv!(stream, buffer);
    match &message.body {
        TransportBody::InitSyn(InitSyn { is_qos, .. }) => {
            let whatami = my_whatami;
            let sn_resolution = None;
            let cookie = vec![0u8; 8].into();
            let attachment = None;
            let message = TransportMessage::make_init_ack(
                whatami,
                my_pid,
                sn_resolution,
                *is_qos,
                cookie,
                attachment,
            );
            // Send the InitAck
            zsend!(&message, stream);
        }
        _ => panic!(),
    }

    // Read the OpenSyn
    let message = zrecv!(stream, buffer);
    match &message.body {
        TransportBody::OpenSyn(OpenSyn {
            lease, initial_sn, ..
        }) => {
            let attachment = None;
            let message = TransportMessage::make_open_ack(*lease, *initial_sn, attachment);
            // Send the OpenAck
            zsend!(&message, stream);
        }
        _ => panic!(),
    }

    // Spawn the loggin task
    let counter = Arc::new(AtomicUsize::new(0));
    let c_c = counter.clone();
    task::spawn(async move {
        loop {
            task::sleep(Duration::from_secs(1)).await;
            let c = c_c.swap(0, Ordering::Relaxed);
            if c > 0 {
                println!("{:.6} Gbit/s", (8_f64 * c as f64) / 1000000000_f64);
            }
        }
    });

    // Spawn the KeepAlive task
    let mut c_stream = stream.clone();
    task::spawn(async move {
        loop {
            task::sleep(Duration::from_secs(1)).await;
            let message = TransportMessage::make_keep_alive(None, None);
            zsend!(&message, c_stream);
        }
    });

    // Read from the socket
    loop {
        let n = stream.read(&mut buffer).await?;
        let _ = counter.fetch_add(n, Ordering::Relaxed);
    }
}

async fn run(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let locator = TcpListener::bind(addr).await?;
    let mut incoming = locator.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        task::spawn(async move {
            let _ = handle_client(stream).await;
        });
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(name = "t_sink_tcp")]
struct Opt {
    #[clap(short, long)]
    listen: SocketAddr,
}

#[async_std::main]
async fn main() {
    env_logger::init();
    let opt = Opt::parse();
    let _ = run(opt.listen).await;
}
