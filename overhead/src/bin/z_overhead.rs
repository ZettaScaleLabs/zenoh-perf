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
use std::time::Duration;
use zenoh::buffers::ZBuf;
use zenoh::config::Config;
use zenoh::config::TimestampingConf;
use zenoh::prelude::r#async::*;
use zenoh_protocol_core::{CongestionControl, EndPoint, WhatAmI};

#[derive(Debug, Parser)]
#[clap(name = "z_overhead")]
struct Opt {
    #[clap(short, long)]
    locator: EndPoint,
    #[clap(short, long)]
    mode: WhatAmI,
    #[clap(short, long)]
    payload: usize,
    #[clap(short, long)]
    print: bool,
    #[clap(short, long, default_value = "1048576")] //1MB in bytes
    total: u64,
    #[clap(short, long, default_value = "0")]
    interval: f64,
}

const KEY_EXPR: &str = "test/overhead";
const BYTES_IN_MB: u64 = 1048576;

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    // Parse the args
    let opt = Opt::from_args();

    let mut config = Config::default();
    config.set_mode(Some(opt.mode.clone())).unwrap();
    config
        .set_timestamping(
            TimestampingConf::new(
                Some(zenoh::config::ModeDependentValue::Unique(false)),
                Some(false),
            )
            .unwrap(),
        )
        .unwrap();

    config.scouting.multicast.set_enabled(Some(false)).unwrap();
    config.connect.endpoints.push(opt.locator.clone());

    let session = zenoh::open(config).res().await.unwrap();

    let reskey = session.declare_keyexpr(KEY_EXPR).res().await.unwrap();
    let publ = session
        .declare_publisher(&reskey)
        .congestion_control(CongestionControl::Block)
        .res()
        .await
        .unwrap();

    let data: ZBuf = (0usize..opt.payload)
        .map(|i| (i % 10) as u8)
        .collect::<Vec<u8>>()
        .into();

    let mut i: u64 = 0;
    let tot: u64 = (opt.total * BYTES_IN_MB) / (opt.payload as u64);

    if opt.print {
        let count = Arc::new(AtomicUsize::new(0));
        let c_count = count.clone();
        task::spawn(async move {
            loop {
                task::sleep(Duration::from_secs(1)).await;
                let c = count.swap(0, Ordering::Relaxed);
                if c > 0 {
                    println!("{} msg/s", c);
                }
            }
        });

        while i < tot {
            publ.put(data.clone()).res().await.unwrap();
            c_count.fetch_add(1, Ordering::Relaxed);
            i += 1;
            task::sleep(Duration::from_secs_f64(opt.interval)).await;
        }
    } else {
        while i < tot {
            publ.put(data.clone()).res().await.unwrap();
            i += 1;
            task::sleep(Duration::from_secs_f64(opt.interval)).await;
        }
    }
}
