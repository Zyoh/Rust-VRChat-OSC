// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use std::{net::UdpSocket, error::Error};
use rosc::{OscType, OscMessage, OscPacket, encoder};

use crate::vrcinput::VRChatOSCInput;

// TODO: Receive OSC messages from VRChat
pub struct VRChatOSC {
    application_binds_to_addr: String,
    // vrchat_sends_to_addr: String,
    vrchat_listens_to_addr: String,
}

impl Default for VRChatOSC {
    fn default() -> Self {
        Self {
            application_binds_to_addr: "127.0.0.1:49999".to_string(),
            // vrchat_sends_to_addr: "127.0.0.1:9001".to_string(),
            vrchat_listens_to_addr: "127.0.0.1:9000".to_string(),
        }
    }
}

impl VRChatOSC {
    fn send_message(&self, addr: &str, args: Vec<OscType>) -> Result<(), Box<dyn Error>> {
        let sock = UdpSocket::bind(&self.application_binds_to_addr)?;

        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: addr.to_string(),
            args,
        }))?;

        sock.send_to(&msg_buf, &self.vrchat_listens_to_addr)?;

        Ok(())
    }

    pub fn send_vrc_input(&self, vrc_input: VRChatOSCInput) -> Result<(), Box<dyn Error>> {
        self.send_message(&vrc_input.to_osc_addr(), vrc_input.to_osc_type())
    }
}
