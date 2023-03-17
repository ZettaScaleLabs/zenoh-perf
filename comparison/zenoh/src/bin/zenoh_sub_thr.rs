//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
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
use zenoh_config::{EndPoint, WhatAmI};

#[derive(Debug, Parser)]
#[clap(name = "zenoh_sub_thr")]
struct Opt {
    #[clap(short, long, value_delimiter = ',')]
    listen: Option<Vec<EndPoint>>,

    #[clap(short, long, value_delimiter = ',')]
    connect: Option<Vec<EndPoint>>,

    /// peer, router, or client
    #[clap(short, long)]
    mode: WhatAmI,

    /// payload size (bytes)
    #[clap(short, long)]
    payload: usize,

    /// configuration file (json5 or yaml)
    #[clap(long = "conf", value_parser)]
    config: Option<PathBuf>,
}

const KEY_EXPR: &str = "test/thr";

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    // Parse the args
    let Opt {
        listen,
        connect,
        mode,
        payload,
        config,
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
            WhatAmI::Peer => {
                if let Some(endpoint) = listen {
                    config.listen.endpoints.extend(endpoint);
                }
                if let Some(endpoint) = connect {
                    config.connect.endpoints.extend(endpoint);
                }
            }
            WhatAmI::Client => {
                if let Some(endpoint) = connect {
                    config.connect.endpoints.extend(endpoint);
                }
            }
            _ => panic!("Unsupported mode: {mode}"),
        };
        config
    };

    let messages = Arc::new(AtomicUsize::new(0));
    let c_messages = messages.clone();

    let session = zenoh::open(config).res().await.unwrap();

    let _sub = session
        .declare_subscriber(KEY_EXPR)
        .callback_mut(move |_| {
            c_messages.fetch_add(1, Ordering::Relaxed);
        })
        .reliable()
        .res()
        .await
        .unwrap();
    measure(messages, payload).await;
}

async fn measure(messages: Arc<AtomicUsize>, payload: usize) {
    let mut timer = Instant::now();
    loop {
        task::sleep(Duration::from_secs(1)).await;

        if messages.load(Ordering::Relaxed) > 0 {
            let elapsed = timer.elapsed().as_micros() as f64;
            let c = messages.swap(0, Ordering::Relaxed);
            println!("{},{:.3}", payload, c as f64 * 1_000_000.0 / elapsed);
            timer = Instant::now()
        }
    }
}
