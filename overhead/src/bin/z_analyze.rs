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

extern crate serde;
use async_std::fs;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::io::Read;
use zenoh::buffers::reader::{HasReader, Reader};
use zenoh::buffers::ZBuf;
use zenoh::prelude::r#async::*;
use zenoh_protocol::proto::{
    FramePayload, MessageReader, TransportBody, TransportMessage, ZenohBody, ZenohMessage,
};

#[derive(Debug, Parser)]
#[clap(name = "zn_analyze")]
struct Opt {
    #[clap(short, long)]
    file: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PcapData {
    #[serde(alias = "_index")]
    pub index: String,
    #[serde(alias = "_type")]
    pub pcap_type: String,
    #[serde(alias = "_score")]
    pub score: Option<String>,
    #[serde(alias = "_source")]
    pub source: Layers,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Layers {
    #[serde(alias = "_layers")]
    pub layers: PcapLayers,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PcapLayers {
    #[serde(alias = "frame.len")]
    pub frame_len: Option<Vec<String>>,
    #[serde(alias = "ip.len")]
    pub ip_len: Option<Vec<String>>,
    #[serde(alias = "tcp.dstport")]
    pub tcp_dest: Option<Vec<String>>,
    #[serde(alias = "tcp.srcport")]
    pub tcp_src: Option<Vec<String>>,
    #[serde(alias = "tcp.payload")]
    pub tcp_payload: Option<Vec<String>>,
}

fn read_transport_messages(mut data: &[u8]) -> Vec<TransportMessage> {
    let mut messages: Vec<TransportMessage> = Vec::with_capacity(1);
    let mut length_bytes = [0u8; 2];
    while data.read_exact(&mut length_bytes).is_ok() {
        let to_read = u16::from_le_bytes(length_bytes) as usize;
        // Read the message
        let mut buffer = vec![0u8; to_read];
        let _ = data.read_exact(&mut buffer).unwrap();

        let zbuf = ZBuf::from(buffer);
        let mut zbuf = zbuf.reader();
        while zbuf.can_read() {
            if let Some(msg) = zbuf.read_transport_message() {
                messages.push(msg)
            }
        }
    }

    messages
}

fn read_zenoh_messages(data: Vec<TransportMessage>) -> Vec<ZenohMessage> {
    let mut messages = vec![];
    for m in data.iter() {
        if let TransportBody::Frame(f) = &m.body {
            if let FramePayload::Messages { messages: msgs } = &f.payload {
                messages.extend_from_slice(msgs);
            }
        }
    }
    messages
}

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    // Parse the args
    let opt = Opt::from_args();

    let contents = fs::read_to_string(opt.file).await.unwrap();
    let pkts: Vec<PcapData> = serde_json::from_str(&contents).unwrap();
    let mut zenoh_data: Vec<u8> = vec![];

    let mut payload_size = 0;
    //let mut total_bytes = 0;
    let mut data_count = 0;

    for pkt in pkts.iter() {
        if let Some(payload) = &pkt.source.layers.tcp_payload {
            let p = payload.first().unwrap();
            let mut d = hex::decode(p).unwrap();
            zenoh_data.append(&mut d);
        }
    }

    println!("Total Size of Zenoh messages: {} bytes", zenoh_data.len());

    let transport_messages = read_transport_messages(zenoh_data.as_slice());

    println!("Total TransportMessages: {}", transport_messages.len());

    let zenoh_messages = read_zenoh_messages(transport_messages);

    println!("Total Zenoh Messages: {}", zenoh_messages.len());

    for m in zenoh_messages.iter() {
        if let ZenohBody::Data(d) = &m.body {
            data_count += 1;
            payload_size += d.payload.len();
        }
    }

    let payload = (payload_size / data_count) as u64;

    println!("Total Data messages: {}", data_count);
    println!("Total Payload: {} bytes", payload_size);
    println!("Per message Payload: {} bytes", payload);
}
