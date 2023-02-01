// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use rosc::OscType;

#[derive(Debug, Clone, PartialEq)]
pub enum VRChatOSCType {
    Bool(bool),
    Float(f32),
    Int(u8),
}

impl VRChatOSCType {
    pub fn to_osc_type(&self) -> OscType {
        match self {
            VRChatOSCType::Bool(value) => OscType::Bool(*value),
            VRChatOSCType::Float(value) => OscType::Float(*value),
            VRChatOSCType::Int(value) => OscType::Int(*value as i32),
        }
    }
}
