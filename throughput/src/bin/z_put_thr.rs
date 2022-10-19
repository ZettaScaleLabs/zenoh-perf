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
use zenoh::config::TimestampingConf;
use zenoh::prelude::r#async::*;
use zenoh_protocol_core::{CongestionControl, EndPoint, WhatAmI};

#[derive(Debug, Parser)]
#[clap(name = "z_put_thr")]
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
    #[clap(long = "conf", parse(from_os_str))]
    config: Option<PathBuf>,

    /// declare a numerical Id for the publisher's key expression
    #[clap(long)]
    use_expr: bool,

    /// declare publication before the publisher
    #[clap(long)]
    declare_publication: bool,
}

const KEY_EXPR: &str = "test/thr";

#[async_std::main]
async fn main() {
    // Initiate logging
    env_logger::init();

    // Parse the args
    let Opt {
        endpoint,
        mode,
        payload,
        print,
        config,
        use_expr,
        declare_publication,
    } = Opt::parse();
    let config = {
        let mut config: Config = if let Some(path) = config {
            Config::from_file(path).unwrap()
        } else {
            Config::default()
        };
        config.set_mode(Some(mode)).unwrap();
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
        config.connect.endpoints.extend(endpoint);
        config
    };

    let value: Value = (0usize..payload)
        .map(|i| (i % 10) as u8)
        .collect::<Vec<u8>>()
        .into();

    let session = zenoh::open(config).res().await.unwrap();

    let count = Arc::new(AtomicUsize::new(0));
    let c_count = count.clone();

    match (use_expr, declare_publication, print) {
        (false, false, false) => loop {
            session
                .put(KEY_EXPR, value.clone())
                .congestion_control(CongestionControl::Block)
                .res()
                .await
                .unwrap();
        },
        (true, false, false) => {
            let expr_id = session.declare_keyexpr(KEY_EXPR).res().await.unwrap();
            loop {
                session
                    .put(&expr_id, value.clone())
                    .congestion_control(CongestionControl::Block)
                    .res()
                    .await
                    .unwrap();
            }
        }
        (true, false, true) => {
            let expr_id = session.declare_keyexpr(KEY_EXPR).res().await.unwrap();
            loop {
                session
                    .put(&expr_id, value.clone())
                    .congestion_control(CongestionControl::Block)
                    .res()
                    .await
                    .unwrap();
                c_count.fetch_add(1, Ordering::Relaxed);
            }
        }
        (false, true, false) => {
            let publisher = session
                .declare_publisher(KEY_EXPR)
                .congestion_control(CongestionControl::Block)
                .res()
                .await
                .unwrap();
            loop {
                publisher.put(value.clone()).res().await.unwrap();
            }
        }
        (true, true, false) => {
            let expr_id = session.declare_keyexpr(KEY_EXPR).res().await.unwrap();
            let publisher = session
                .declare_publisher(expr_id)
                .congestion_control(CongestionControl::Block)
                .res()
                .await
                .unwrap();
            loop {
                publisher.put(value.clone()).res().await.unwrap();
            }
        }
        (true, true, true) => {
            let expr_id = session.declare_keyexpr(KEY_EXPR).res().await.unwrap();
            let publisher = session
                .declare_publisher(expr_id)
                .congestion_control(CongestionControl::Block)
                .res()
                .await
                .unwrap();
            loop {
                publisher.put(value.clone()).res().await.unwrap();
                c_count.fetch_add(1, Ordering::Relaxed);
            }
        }
        (_, _, _) => (),
    }

    if print {
        task::spawn(async move {
            loop {
                task::sleep(Duration::from_secs(1)).await;
                let c = count.swap(0, Ordering::Relaxed);
                if c > 0 {
                    println!("{} msg/s", c);
                }
            }
        });
    }
}
