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
use async_std::{future, sync::Arc, task};
use clap::Parser;
use std::{
    any::Any,
    path::PathBuf,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    time::{Duration, Instant},
};
use zenoh::config::Config;
use zenoh_core::zresult::ZResult;
use zenoh_link::Link;
use zenoh_protocol::{
    core::{CongestionControl, EndPoint, Priority, WhatAmI, WireExpr},
    network::{
        declare::{
            ext::{NodeIdType, QoSType},
            subscriber::ext::SubscriberInfo,
        },
        Declare, DeclareBody, DeclareSubscriber, NetworkMessage,
    },
};
use zenoh_transport::*;

// Transport Handler for the peer
struct MySH {
    scenario: String,
    name: String,
    payload: usize,
    counter: Arc<AtomicUsize>,
    active: AtomicBool,
}

impl MySH {
    fn new(scenario: String, name: String, payload: usize, counter: Arc<AtomicUsize>) -> Self {
        Self {
            scenario,
            name,
            payload,
            counter,
            active: AtomicBool::new(false),
        }
    }
}

impl TransportEventHandler for MySH {
    fn new_unicast(
        &self,
        _peer: TransportPeer,
        transport: TransportUnicast,
    ) -> ZResult<Arc<dyn TransportPeerEventHandler>> {
        // Declare Subscriber
        let message: NetworkMessage = Declare {
            ext_qos: QoSType::new(Priority::default(), CongestionControl::Block, false),
            ext_tstamp: None,
            ext_nodeid: NodeIdType::default(),
            body: DeclareBody::DeclareSubscriber(DeclareSubscriber {
                id: 0,
                wire_expr: WireExpr::from("test/thr"),
                ext_info: SubscriberInfo::default(),
            }),
        }
        .into();
        transport.handle_message(message.clone()).unwrap();

        if !self.active.swap(true, Ordering::Acquire) {
            let count = self.counter.clone();
            let scenario = self.scenario.clone();
            let name = self.name.clone();
            let payload = self.payload;
            task::spawn(async move {
                loop {
                    let now = Instant::now();
                    task::sleep(Duration::from_secs(1)).await;
                    let elapsed = now.elapsed().as_micros() as f64;

                    let c = count.swap(0, Ordering::Relaxed);
                    if c > 0 {
                        let interval = 1_000_000.0 / elapsed;
                        println!(
                            "session,{},throughput,{},{},{}",
                            scenario,
                            name,
                            payload,
                            (c as f64 / interval).floor() as usize
                        );
                    }
                }
            });
        }
        Ok(Arc::new(MyMH::new(self.counter.clone())))
    }
}

// Message Handler for the peer
struct MyMH {
    counter: Arc<AtomicUsize>,
}

impl MyMH {
    fn new(counter: Arc<AtomicUsize>) -> Self {
        Self { counter }
    }
}

impl TransportPeerEventHandler for MyMH {
    fn handle_message(&self, _message: NetworkMessage) -> ZResult<()> {
        self.counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn new_link(&self, _link: Link) {}
    fn del_link(&self, _link: Link) {}
    fn closing(&self) {}
    fn closed(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Parser)]
#[clap(name = "t_sub_thr")]
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

    #[clap(short, long)]
    name: String,

    #[clap(short, long)]
    scenario: String,

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
        name,
        scenario,
        config,
    } = Opt::parse();

    // Setup TransportManager
    let count = Arc::new(AtomicUsize::new(0));
    let builder = match config {
        Some(path) => TransportManager::builder()
            .from_config(&Config::from_file(path).unwrap())
            .await
            .unwrap(),
        None => TransportManager::builder().whatami(WhatAmI::Router),
    };
    let handler = Arc::new(MySH::new(scenario, name, payload, count));
    let manager = builder.build(handler).unwrap();

    if mode == WhatAmI::Peer {
        for e in endpoint {
            manager.add_listener(e.clone()).await.unwrap();
        }
    } else {
        for e in endpoint {
            let _t = manager.open_transport_unicast(e.clone()).await.unwrap();
        }
    }

    // Stop forever
    future::pending::<()>().await;
}
