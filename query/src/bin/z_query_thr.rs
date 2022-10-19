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

use async_std::sync::Arc;
use async_std::task;
use clap::Parser;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use zenoh::prelude::r#async::*;
use zenoh_protocol_core::{EndPoint, WhatAmI};

#[derive(Debug, Parser)]
#[clap(name = "z_query_thr")]
struct Opt {
    #[clap(short, long)]
    locator: EndPoint,
    #[clap(short, long)]
    mode: WhatAmI,
    #[clap(short, long)]
    name: String,
    #[clap(short, long)]
    scenario: String,
    #[clap(short, long)]
    payload: usize,
}

const KEY_EXPR: &str = "test/query";

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    // Parse the args
    let opt = Opt::from_args();
    let mut config: Config = Config::default();
    config.set_mode(Some(opt.mode.clone())).unwrap();
    config.scouting.multicast.set_enabled(Some(false)).unwrap();
    config.connect.endpoints.push(opt.locator.clone());

    let session = zenoh::open(config).res().await.unwrap();

    let rtt = Arc::new(AtomicUsize::new(0));
    let counter = Arc::new(AtomicUsize::new(0));

    let c_rtt = rtt.clone();
    let c_counter = counter.clone();
    task::spawn(async move {
        loop {
            let now = Instant::now();
            task::sleep(Duration::from_secs(1)).await;
            let elapsed = now.elapsed().as_secs_f64();

            let r = c_rtt.swap(0, Ordering::Relaxed);
            let c = c_counter.swap(0, Ordering::Relaxed);
            if c > 0 {
                println!(
                    "zenoh,{},query.throughput,{},{},{},{}",
                    opt.scenario,
                    opt.name,
                    opt.payload,
                    (c as f64 / elapsed).floor() as usize,
                    (r as f64 / c as f64).floor() as usize,
                );
            }
        }
    });

    loop {
        let now = Instant::now();
        let data_stream = session.get(KEY_EXPR).res().await.unwrap();
        while data_stream.recv_async().await.is_ok() {}

        rtt.fetch_add(now.elapsed().as_micros() as usize, Ordering::Relaxed);
        counter.fetch_add(1, Ordering::Relaxed);
    }
}
