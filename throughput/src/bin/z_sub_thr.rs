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
    time::{Duration, Instant},
};
use zenoh::config::Config;
use zenoh::prelude::r#async::*;
use zenoh_protocol_core::{EndPoint, WhatAmI};

#[derive(Debug, Parser)]
#[clap(name = "z_sub_thr")]
struct Opt {
    /// endpoint(s), e.g. --endpoint tcp/127.0.0.1:7447,tcp/127.0.0.1:7448
    #[clap(short, long, required(true), value_delimiter = ',')]
    endpoint: Vec<EndPoint>,

    /// peer, router, or client
    #[clap(short, long, possible_values = ["peer", "client"])]
    mode: WhatAmI,

    /// payload size (bytes)
    #[clap(short, long)]
    payload: usize,

    #[clap(short, long)]
    name: String,

    #[clap(short, long)]
    scenario: String,

    /// configuration file (json5 or yaml)
    #[clap(long = "conf", parse(from_os_str))]
    config: Option<PathBuf>,

    /// declare a numerical Id for the subscribed key expression
    #[clap(long)]
    use_expr: bool,

    /// do not use callback for subscriber
    #[clap(long)]
    no_callback: bool,
}

const KEY_EXPR: &str = "test/thr";

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    // Parse the args
    let Opt {
        endpoint,
        mode,
        payload,
        name,
        scenario,
        config,
        use_expr,
        no_callback,
    } = Opt::parse();

    let config = {
        let mut config: Config = if let Some(path) = config {
            Config::from_file(path).unwrap()
        } else {
            Config::default()
        };
        config.set_mode(Some(mode)).unwrap();
        config.scouting.multicast.set_enabled(Some(false)).unwrap();
        match mode {
            WhatAmI::Peer => config.listen.endpoints.extend(endpoint),
            WhatAmI::Client => config.connect.endpoints.extend(endpoint),
            _ => panic!("Unsupported mode: {}", mode),
        };
        config
    };

    let messages = Arc::new(AtomicUsize::new(0));
    let c_messages = messages.clone();

    let session = zenoh::open(config).res().await.unwrap();
    let sub_builder = if use_expr {
        session.declare_subscriber(KEY_EXPR)
    } else {
        session.declare_subscriber(session.declare_keyexpr(KEY_EXPR).res().await.unwrap())
    };

    if no_callback {
        task::spawn(async move {
            measure(c_messages, scenario, name, payload).await;
        });

        let subscriber = sub_builder.reliable().push_mode().res().await.unwrap();

        while subscriber.recv().is_ok() {
            messages.fetch_add(1, Ordering::Relaxed);
        }
    } else {
        let _subscriber = sub_builder
            .callback(move |_| {
                c_messages.fetch_add(1, Ordering::Relaxed);
            })
            .reliable()
            .push_mode()
            .res()
            .await
            .unwrap();

        measure(messages, scenario, name, payload).await;
    }
}

async fn measure(messages: Arc<AtomicUsize>, scenario: String, name: String, payload: usize) {
    loop {
        let now = Instant::now();
        task::sleep(Duration::from_secs(1)).await;
        let elapsed = now.elapsed().as_micros() as f64;

        let c = messages.swap(0, Ordering::Relaxed);
        if c > 0 {
            let interval = 1_000_000.0 / elapsed;
            println!(
                "zenoh,{},throughput,{},{},{}",
                scenario,
                name,
                payload,
                (c as f64 / interval).floor() as usize
            );
        }
    }
}
