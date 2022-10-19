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
use async_std::future;
use clap::Parser;
use zenoh::config::Config;
use zenoh::prelude::r#async::*;
use zenoh_protocol_core::{CongestionControl, WhatAmI};

#[derive(Debug, Parser)]
#[clap(name = "z_pong")]
struct Opt {
    /// endpoint(s), e.g. --endpoint tcp/127.0.0.1:7447,tcp/127.0.0.1:7448
    #[clap(short, long)]
    endpoint: String,

    /// peer or client
    #[clap(short, long, possible_values = ["peer", "client"])]
    mode: String,

    /// declare a numerical ID for key expression
    #[clap(long)]
    use_expr: bool,

    /// declare publication before the publisher
    #[clap(long)]
    declare_publication: bool,
}

const KEY_EXPR_PING: &str = "test/z_ping";
const KEY_EXPR_PONG: &str = "test/z_pong";

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    // Parse the args
    let opt = Opt::parse();

    let mut config = Config::default();
    match opt.mode.as_str() {
        "peer" => {
            config.set_mode(Some(WhatAmI::Peer)).unwrap();
            config
                .listen
                .endpoints
                .extend(opt.endpoint.split(',').map(|v| v.parse().unwrap()));
        }
        "client" => {
            config.set_mode(Some(WhatAmI::Client)).unwrap();
            config
                .connect
                .endpoints
                .extend(opt.endpoint.split(',').map(|v| v.parse().unwrap()));
        }
        _ => panic!("Unsupported mode: {}", opt.mode),
    };
    config.scouting.multicast.set_enabled(Some(false)).unwrap();

    let session = zenoh::open(config).res().await.unwrap();

    let key_expr_pong = if opt.use_expr {
        Some(session.declare_keyexpr(KEY_EXPR_PONG).res().await.unwrap())
    } else {
        None
    };

    let publisher = if opt.declare_publication {
        if key_expr_pong.is_some() {
            Some(
                session
                    .declare_publisher(key_expr_pong.clone().unwrap())
                    .congestion_control(CongestionControl::Block)
                    .res()
                    .await
                    .unwrap(),
            )
        } else {
            Some(
                session
                    .declare_publisher(KEY_EXPR_PONG)
                    .congestion_control(CongestionControl::Block)
                    .res()
                    .await
                    .unwrap(),
            )
        }
    } else {
        None
    };

    let sub = if opt.use_expr {
        // Declare the subscriber
        let key_expr_ping = session.declare_keyexpr(KEY_EXPR_PING).res().await.unwrap();
        session
            .declare_subscriber(key_expr_ping)
            .reliable()
            .res()
            .await
            .unwrap()
    } else {
        session
            .declare_subscriber(KEY_EXPR_PING)
            .reliable()
            .res()
            .await
            .unwrap()
    };

    while let Ok(sample) = sub.recv_async().await {
        if publisher.as_ref().is_some() {
            publisher.as_ref().unwrap().put(sample).res().await.unwrap();
        } else {
            if key_expr_pong.as_ref().is_some() {
                session
                    .put(key_expr_pong.as_ref().unwrap(), sample)
                    .res()
                    .await
                    .unwrap();
            } else {
                session
                    .put(KEY_EXPR_PONG, sample)
                    .congestion_control(CongestionControl::Block)
                    .res()
                    .await
                    .unwrap();
            }
        }
    }

    // Stop forever
    future::pending::<()>().await;
}
