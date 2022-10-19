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
use async_std::sync::{Arc, Mutex};
use async_std::task;
use clap::Parser;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use zenoh::buffers::reader::{HasReader, Reader};
use zenoh::config::Config;
use zenoh::prelude::r#async::*;
use zenoh_protocol_core::{CongestionControl, WhatAmI};

#[derive(Debug, Parser)]
#[clap(name = "z_ping")]
struct Opt {
    /// endpoint(s), e.g. --endpoint tcp/127.0.0.1:7447,tcp/127.0.0.1:7448
    #[clap(short, long, required(true))]
    endpoint: Option<String>,

    /// peer or client
    #[clap(short, long, possible_values = ["peer", "client"])]
    mode: String,

    /// payload size (bytes)
    #[clap(short, long)]
    payload: usize,

    #[clap(short, long)]
    name: String,

    #[clap(short, long)]
    scenario: String,

    /// interval of sending message (sec)
    #[clap(short, long)]
    interval: f64,

    /// spawn a task to receive or not
    #[clap(long = "parallel")]
    parallel: bool,

    /// declare a numerical ID for key expression
    #[clap(long)]
    use_expr: bool,

    /// declare publication before the publisher
    #[clap(long)]
    declare_publication: bool,
}

const KEY_EXPR_PING: &str = "test/z_ping";
const KEY_EXPR_PONG: &str = "test/z_pong";

async fn parallel(opt: Opt, config: Config) {
    let session = zenoh::open(config).res().await.unwrap();
    let session = Arc::new(session);

    // The hashmap with the pings
    let pending = Arc::new(Mutex::new(HashMap::<u64, Instant>::new()));

    let c_pending = pending.clone();
    let scenario = opt.scenario;
    let name = opt.name;
    let interval = opt.interval;

    let sub = if opt.use_expr {
        // Declare the subscriber
        let key_expr_pong = session.declare_keyexpr(KEY_EXPR_PONG).res().await.unwrap();
        session
            .declare_subscriber(key_expr_pong)
            .reliable()
            .res()
            .await
            .unwrap()
    } else {
        session
            .declare_subscriber(KEY_EXPR_PONG)
            .reliable()
            .res()
            .await
            .unwrap()
    };

    task::spawn(async move {
        while let Ok(sample) = sub.recv_async().await {
            let mut payload_reader = sample.value.payload.reader();
            let mut count_bytes = [0u8; 8];
            if payload_reader.read_exact(&mut count_bytes) {
                let count = u64::from_le_bytes(count_bytes);

                let instant = c_pending.lock().await.remove(&count).unwrap();
                println!(
                    "zenoh,{},latency.parallel,{},{},{},{},{}",
                    scenario,
                    name,
                    sample.value.payload.len(),
                    interval,
                    count,
                    instant.elapsed().as_micros()
                );
            } else {
                panic!("Fail to fill the buffer");
            }
        }
        panic!("Invalid value!");
    });

    let key_expr_ping = if opt.use_expr {
        Some(session.declare_keyexpr(KEY_EXPR_PING).res().await.unwrap())
    } else {
        None
    };

    let publisher = if opt.declare_publication {
        if key_expr_ping.is_some() {
            Some(
                session
                    .declare_publisher(key_expr_ping.clone().unwrap())
                    .congestion_control(CongestionControl::Block)
                    .res()
                    .await
                    .unwrap(),
            )
        } else {
            Some(
                session
                    .declare_publisher(KEY_EXPR_PING)
                    .congestion_control(CongestionControl::Block)
                    .res()
                    .await
                    .unwrap(),
            )
        }
    } else {
        None
    };

    let mut count: u64 = 0;
    loop {
        let count_bytes: [u8; 8] = count.to_le_bytes();
        let mut payload = vec![0u8; opt.payload];
        payload[0..8].copy_from_slice(&count_bytes);

        pending.lock().await.insert(count, Instant::now());

        if publisher.as_ref().is_some() {
            publisher
                .as_ref()
                .unwrap()
                .put(payload)
                .res()
                .await
                .unwrap();
        } else {
            if key_expr_ping.as_ref().is_some() {
                session
                    .put(key_expr_ping.as_ref().unwrap(), payload)
                    .res()
                    .await
                    .unwrap();
            } else {
                session
                    .put(KEY_EXPR_PING, payload)
                    .congestion_control(CongestionControl::Block)
                    .res()
                    .await
                    .unwrap();
            }
        }
        task::sleep(Duration::from_secs_f64(opt.interval)).await;
        count += 1;
    }
}

async fn single(opt: Opt, config: Config) {
    let session = zenoh::open(config).res().await.unwrap();

    let scenario = opt.scenario;
    let name = opt.name;
    let interval = opt.interval;

    let sub = if opt.use_expr {
        // Declare the subscriber
        let key_expr_pong = session.declare_keyexpr(KEY_EXPR_PONG).res().await.unwrap();
        session
            .declare_subscriber(&key_expr_pong)
            .reliable()
            .res()
            .await
            .unwrap()
    } else {
        session
            .declare_subscriber(KEY_EXPR_PONG)
            .reliable()
            .res()
            .await
            .unwrap()
    };

    let key_expr_ping = if opt.use_expr {
        Some(session.declare_keyexpr(KEY_EXPR_PING).res().await.unwrap())
    } else {
        None
    };

    let publisher = if opt.declare_publication {
        if key_expr_ping.is_some() {
            Some(
                session
                    .declare_publisher(key_expr_ping.clone().unwrap())
                    .congestion_control(CongestionControl::Block)
                    .res()
                    .await
                    .unwrap(),
            )
        } else {
            Some(
                session
                    .declare_publisher(KEY_EXPR_PING)
                    .congestion_control(CongestionControl::Block)
                    .res()
                    .await
                    .unwrap(),
            )
        }
    } else {
        None
    };

    let mut count: u64 = 0;
    loop {
        let count_bytes: [u8; 8] = count.to_le_bytes();
        let mut payload = vec![0u8; opt.payload];
        payload[0..8].copy_from_slice(&count_bytes);

        let now = Instant::now();

        if publisher.as_ref().is_some() {
            publisher
                .as_ref()
                .unwrap()
                .put(payload)
                .res()
                .await
                .unwrap();
        } else {
            if key_expr_ping.as_ref().is_some() {
                session
                    .put(key_expr_ping.as_ref().unwrap(), payload)
                    .res()
                    .await
                    .unwrap();
            } else {
                session
                    .put(KEY_EXPR_PING, payload)
                    .congestion_control(CongestionControl::Block)
                    .res()
                    .await
                    .unwrap();
            }
        }

        match sub.recv_async().await {
            Ok(sample) => {
                let mut payload_reader = sample.value.payload.reader();
                let mut count_bytes = [0u8; 8];
                if payload_reader.read_exact(&mut count_bytes) {
                    let s_count = u64::from_le_bytes(count_bytes);

                    println!(
                        "zenoh,{},latency.sequential,{},{},{},{},{}",
                        scenario,
                        name,
                        sample.value.payload.len(),
                        interval,
                        s_count,
                        now.elapsed().as_micros()
                    );
                } else {
                    panic!("Fail to fill the buffer");
                }
            }
            _ => panic!("Invalid value"),
        }
        task::sleep(Duration::from_secs_f64(opt.interval)).await;
        count += 1;
    }
}

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    // Parse the args
    let opt = Opt::parse();

    let mut config = Config::default();
    match opt.mode.as_str() {
        "peer" => config.set_mode(Some(WhatAmI::Peer)).unwrap(),
        "client" => config.set_mode(Some(WhatAmI::Client)).unwrap(),
        _ => panic!("Unsupported mode: {}", opt.mode),
    };

    if let Some(ref l) = opt.endpoint {
        config.scouting.multicast.set_enabled(Some(false)).unwrap();
        config
            .connect
            .endpoints
            .extend(l.split(',').map(|v| v.parse().unwrap()));
    } else {
        config.scouting.multicast.set_enabled(Some(true)).unwrap();
    }

    if opt.parallel {
        parallel(opt, config).await;
    } else {
        single(opt, config).await;
    }
}
