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
use async_std::{sync::Arc, task};
use clap::Parser;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use zenoh::config::Config;
use zenoh_buffers::ZBuf;
use zenoh_core::zresult::ZResult;
use zenoh_protocol::{
    core::{Channel, CongestionControl, EndPoint, Priority, Reliability, WhatAmI, WireExpr},
    zenoh::ZenohMessage,
};
use zenoh_transport::*;

struct MySH {}

impl MySH {
    fn new() -> Self {
        Self {}
    }
}

impl TransportEventHandler for MySH {
    fn new_unicast(
        &self,
        _peer: TransportPeer,
        _transport: TransportUnicast,
    ) -> ZResult<Arc<dyn TransportPeerEventHandler>> {
        Ok(Arc::new(DummyTransportPeerEventHandler::default()))
    }

    fn new_multicast(
        &self,
        _transport: TransportMulticast,
    ) -> ZResult<Arc<dyn TransportMulticastEventHandler>> {
        panic!();
    }
}

#[derive(Debug, Parser)]
#[clap(name = "t_pub_thr")]
struct Opt {
    /// endpoint(s), e.g. --endpoint tcp/127.0.0.1:7447,tcp/127.0.0.1:7448
    #[clap(short, long, required(true), value_delimiter = ',')]
    endpoint: Vec<EndPoint>,

    /// peer, router, or client
    #[clap(short, long)]
    mode: WhatAmI,

    /// payload size (bytes)
    #[clap(short, long)]
    payload: usize,

    /// print the counter
    #[clap(short = 't', long)]
    print: bool,

    /// configuration file (json5 or yaml)
    #[clap(long = "conf")]
    config: Option<PathBuf>,
}

#[async_std::main]
async fn main() {
    // Enable logging
    env_logger::init();

    // Parse the args
    let Opt {
        endpoint,
        mode,
        payload,
        print,
        config,
    } = Opt::parse();

    // Setup TransportManager
    let builder = match config {
        Some(path) => TransportManager::builder()
            .from_config(&Config::from_file(path).unwrap())
            .await
            .unwrap(),
        None => TransportManager::builder().whatami(mode),
    };
    let handler = Arc::new(MySH::new());
    let manager = builder.build(handler).unwrap();

    // Connect to publisher
    let mut transports: Vec<TransportUnicast> = vec![];
    for e in endpoint {
        let t = manager.open_transport_unicast(e.clone()).await.unwrap();
        transports.push(t);
    }

    // Send reliable messages
    let channel = Channel {
        priority: Priority::Data,
        reliability: Reliability::Reliable,
    };
    let congestion_control = CongestionControl::Block;
    let key = WireExpr::from(1);
    let info = None;
    let payload = ZBuf::from(vec![0u8; payload]);
    let reply_context = None;
    let routing_context = None;
    let attachment = None;

    let count = Arc::new(AtomicUsize::new(0));
    if print {
        let c_count = count.clone();
        task::spawn(async move {
            loop {
                task::sleep(Duration::from_secs(1)).await;
                let c = c_count.swap(0, Ordering::Relaxed);
                if c > 0 {
                    println!("{} msg/s", c);
                }
            }
        });
    }

    loop {
        for t in transports.iter() {
            let message = ZenohMessage::make_data(
                key.clone(),
                payload.clone(),
                channel,
                congestion_control,
                info.clone(),
                routing_context,
                reply_context.clone(),
                attachment.clone(),
            );
            t.handle_message(message).unwrap();
        }
        count.fetch_add(1, Ordering::Relaxed);
    }
}
