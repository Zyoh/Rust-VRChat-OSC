// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use crate::input::Input;

use std::{net::{UdpSocket, SocketAddrV4}, error::Error, str::FromStr};
use rosc::{OscMessage, OscPacket, encoder};

pub struct Engine {
    pub application_binds_to_addr: String,
    pub vrchat_sends_to_addr: String,
    pub vrchat_listens_to_addr: String,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            application_binds_to_addr: "127.0.0.1:49999".to_string(),
            vrchat_sends_to_addr: "127.0.0.1:9001".to_string(),
            vrchat_listens_to_addr: "127.0.0.1:9000".to_string(),
        }
    }
}

impl Engine {
    fn send_message(&self, message: OscMessage) -> Result<(), Box<dyn Error>> {
        let sock = UdpSocket::bind(&self.application_binds_to_addr)?;
        let msg_buf = encoder::encode(&OscPacket::Message(message))?;
        sock.send_to(&msg_buf, &self.vrchat_listens_to_addr)?;

        Ok(())
    }

    pub fn send(&self, message: Input) -> Result<(), Box<dyn Error>> {
        self.send_message(message.into())
    }

    pub fn listen(&self, callback: &dyn Fn(OscMessage)) -> Result<(), Box<dyn Error>> {
        let addr = SocketAddrV4::from_str(&self.vrchat_sends_to_addr)?;
        let sock = UdpSocket::bind(addr).unwrap();
        let mut buf = [0u8; rosc::decoder::MTU];

        loop {
            let (size, _) = sock.recv_from(&mut buf)?;
            let (_, packet) = rosc::decoder::decode_udp(&buf[..size])?;
            if let OscPacket::Message(msg) = packet {
                callback(msg);
            }
        }
    }
}

