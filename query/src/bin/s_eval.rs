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
use async_std::sync::Arc;
use async_trait::async_trait;
use rand::RngCore;
use structopt::StructOpt;
use zenoh::net::protocol::core::{whatami, CongestionControl, PeerId, Reliability, ResKey};
use zenoh::net::protocol::io::RBuf;
use zenoh::net::protocol::link::{Link, Locator};
use zenoh::net::protocol::proto::ZenohMessage;
use zenoh::net::protocol::session::{
    Session, SessionDispatcher, SessionEventHandler, SessionHandler, SessionManager,
    SessionManagerConfig,
};
use zenoh_util::core::ZResult;

// Session Handler for the peer
struct MySH {
    payload: usize,
}

impl MySH {
    fn new(payload: usize) -> Self {
        Self { payload }
    }
}

#[async_trait]
impl SessionHandler for MySH {
    async fn new_session(
        &self,
        session: Session,
    ) -> ZResult<Arc<dyn SessionEventHandler + Send + Sync>> {
        Ok(Arc::new(MyMH::new(session, self.payload)))
    }
}

// Message Handler for the peer
struct MyMH {
    session: Session,
    payload: usize,
}

impl MyMH {
    fn new(session: Session, payload: usize) -> Self {
        Self { session, payload }
    }
}

#[async_trait]
impl SessionEventHandler for MyMH {
    async fn handle_message(&self, _message: ZenohMessage) -> ZResult<()> {
        // Send reliable messages
        let reliability = Reliability::Reliable;
        let congestion_control = CongestionControl::Block;
        let key = ResKey::RName("test".to_string());
        let info = None;
        let payload = RBuf::from(vec![0u8; self.payload]);
        let reply_context = None;
        let routing_context = None;
        let attachment = None;

        let message = ZenohMessage::make_data(
            key,
            payload,
            reliability,
            congestion_control,
            info,
            routing_context,
            reply_context,
            attachment,
        );
        self.session.handle_message(message).await
    }

    async fn new_link(&self, _link: Link) {}
    async fn del_link(&self, _link: Link) {}
    async fn closing(&self) {}
    async fn closed(&self) {}
}

#[derive(Debug, StructOpt)]
#[structopt(name = "s_sub_thr")]
struct Opt {
    #[structopt(short = "l", long = "locator")]
    locator: Locator,
    #[structopt(short = "m", long = "mode")]
    mode: String,
    #[structopt(short = "p", long = "payload")]
    payload: usize,
}

#[async_std::main]
async fn main() {
    // Enable logging
    env_logger::init();

    // Parse the args
    let opt = Opt::from_args();

    let whatami = match opt.mode.as_str() {
        "peer" => whatami::PEER,
        "client" => whatami::CLIENT,
        _ => panic!("Unsupported mode: {}", opt.mode),
    };

    // Initialize the Peer Id
    let mut pid = [0u8; PeerId::MAX_SIZE];
    rand::thread_rng().fill_bytes(&mut pid);
    let pid = PeerId::new(1, pid);

    let config = SessionManagerConfig {
        version: 0,
        whatami,
        id: pid,
        handler: SessionDispatcher::SessionHandler(Arc::new(MySH::new(opt.payload))),
    };
    let manager = SessionManager::new(config, None);

    // Connect to the peer or listen
    if whatami == whatami::PEER {
        manager.add_listener(&opt.locator).await.unwrap();
    } else {
        let _session = manager.open_session(&opt.locator).await.unwrap();
    }

    // Stop forever
    future::pending::<()>().await;
}