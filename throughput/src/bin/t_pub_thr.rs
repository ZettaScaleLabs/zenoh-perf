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
use zenoh_core::zresult::ZResult;
use zenoh_protocol::{
    core::{CongestionControl, Encoding, EndPoint, Priority, WhatAmI, WireExpr},
    network::{
        push::ext::{NodeIdType, QoSType},
        Declare, DeclareBody, DeclareKeyExpr, Mapping, NetworkMessage, Push,
    },
    zenoh_new::Put,
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
        Ok(Arc::new(DummyTransportPeerEventHandler))
    }

    fn new_multicast(
        &self,
        _: TransportMulticast,
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

    // Declare WireExpr
    let message: NetworkMessage = Declare {
        ext_qos: QoSType::new(Priority::default(), CongestionControl::Block, false),
        ext_tstamp: None,
        ext_nodeid: NodeIdType::default(),
        body: DeclareBody::DeclareKeyExpr(DeclareKeyExpr {
            id: 1,
            wire_expr: WireExpr {
                scope: 0,
                suffix: "test/thr".into(),
                mapping: Mapping::Sender,
            },
        }),
    }
    .into();
    for t in transports.iter() {
        t.schedule(message.clone()).unwrap();
    }

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

    // Send reliable messages
    let message: NetworkMessage = Push {
        wire_expr: WireExpr::from(1),
        ext_qos: QoSType::new(Priority::default(), CongestionControl::Block, false),
        ext_tstamp: None,
        ext_nodeid: NodeIdType::default(),
        payload: Put {
            payload: vec![0u8; payload].into(),
            timestamp: None,
            encoding: Encoding::default(),
            ext_sinfo: None,
            ext_unknown: vec![],
        }
        .into(),
    }
    .into();

    loop {
        for t in transports.iter() {
            t.schedule(message.clone()).unwrap();
        }
        count.fetch_add(1, Ordering::Relaxed);
    }
}
