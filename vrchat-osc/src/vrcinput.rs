// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use crate::VRChatOSCType;

use rosc::OscType;

pub enum VRChatOSCInput {
    Vertical(f32),
    Horizontal(f32),
    LookHorizontal(f32),
    UseAxisRight(f32),
    GrabAxisRight(f32),
    MoveHoldFB(f32),
    SpinHoldCwCcw(f32),
    SpinHoldUD(f32),
    SpinHoldLR(f32),

    MoveForward(bool),
    MoveBackward(bool),
    MoveLeft(bool),
    MoveRight(bool),
    LookLeft(bool),
    LookRight(bool),
    Jump(bool),
    Run(bool),
    ComfortLeft(bool),
    ComfortRight(bool),
    DropRight(bool),
    UseRight(bool),
    GrabRight(bool),
    DropLeft(bool),
    UseLeft(bool),
    GrabLeft(bool),
    PanicButton(bool),
    QuickMenuToggleLeft(bool),
    QuickMenuToggleRight(bool),
    Voice(bool),

    ChatboxInput(String, bool),
    ChatboxTyping(bool),

    Avatar(String, VRChatOSCType),
    Custom(String, Vec<OscType>),
}

impl VRChatOSCInput {
    pub fn to_osc_type(&self) -> Vec<OscType> {
        match self {
            VRChatOSCInput::Vertical(value) => vec![OscType::Float(crate::contrain_value(*value))],
            VRChatOSCInput::Horizontal(value) => vec![OscType::Float(crate::contrain_value(*value))],
            VRChatOSCInput::LookHorizontal(value) => vec![OscType::Float(crate::contrain_value(*value))],
            VRChatOSCInput::UseAxisRight(value) => vec![OscType::Float(crate::contrain_value(*value))],
            VRChatOSCInput::GrabAxisRight(value) => vec![OscType::Float(crate::contrain_value(*value))],
            VRChatOSCInput::MoveHoldFB(value) => vec![OscType::Float(crate::contrain_value(*value))],
            VRChatOSCInput::SpinHoldCwCcw(value) => vec![OscType::Float(crate::contrain_value(*value))],
            VRChatOSCInput::SpinHoldUD(value) => vec![OscType::Float(crate::contrain_value(*value))],
            VRChatOSCInput::SpinHoldLR(value) => vec![OscType::Float(crate::contrain_value(*value))],

            VRChatOSCInput::MoveForward(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::MoveBackward(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::MoveLeft(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::MoveRight(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::LookLeft(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::LookRight(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::Jump(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::Run(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::ComfortLeft(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::ComfortRight(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::DropRight(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::UseRight(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::GrabRight(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::DropLeft(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::UseLeft(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::GrabLeft(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::PanicButton(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::QuickMenuToggleLeft(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::QuickMenuToggleRight(value) => vec![OscType::Bool(*value)],
            VRChatOSCInput::Voice(value) => vec![OscType::Bool(*value)],

            VRChatOSCInput::ChatboxInput(value, immediate) => vec![OscType::String(value.to_owned()), OscType::Bool(*immediate)],
            VRChatOSCInput::ChatboxTyping(value) => vec![OscType::Bool(*value)],

            VRChatOSCInput::Avatar(_, value) => vec![value.to_osc_type()],
            VRChatOSCInput::Custom(_, value) => value.to_owned(),
        }
    }

    pub fn to_osc_addr(&self) -> String {
        match self {
            VRChatOSCInput::Vertical(_) => "/input/Vertical".to_string(),
            VRChatOSCInput::Horizontal(_) => "/input/Horizontal".to_string(),
            VRChatOSCInput::LookHorizontal(_) => "/input/LookHorizontal".to_string(),
            VRChatOSCInput::UseAxisRight(_) => "/input/UseAxisRight".to_string(),
            VRChatOSCInput::GrabAxisRight(_) => "/input/GrabAxisRight".to_string(),
            VRChatOSCInput::MoveHoldFB(_) => "/input/MoveHoldFB".to_string(),
            VRChatOSCInput::SpinHoldCwCcw(_) => "/input/SpinHoldCwCcw".to_string(),
            VRChatOSCInput::SpinHoldUD(_) => "/input/SpinHoldUD".to_string(),
            VRChatOSCInput::SpinHoldLR(_) => "/input/SpinHoldLR".to_string(),

            VRChatOSCInput::MoveForward(_) => "/input/MoveForward".to_string(),
            VRChatOSCInput::MoveBackward(_) => "/input/MoveBackward".to_string(),
            VRChatOSCInput::MoveLeft(_) => "/input/MoveLeft".to_string(),
            VRChatOSCInput::MoveRight(_) => "/input/MoveRight".to_string(),
            VRChatOSCInput::LookLeft(_) => "/input/LookLeft".to_string(),
            VRChatOSCInput::LookRight(_) => "/input/LookRight".to_string(),
            VRChatOSCInput::Jump(_) => "/input/Jump".to_string(),
            VRChatOSCInput::Run(_) => "/input/Run".to_string(),
            VRChatOSCInput::ComfortLeft(_) => "/input/ComfortLeft".to_string(),
            VRChatOSCInput::ComfortRight(_) => "/input/ComfortRight".to_string(),
            VRChatOSCInput::DropRight(_) => "/input/DropRight".to_string(),
            VRChatOSCInput::UseRight(_) => "/input/UseRight".to_string(),
            VRChatOSCInput::GrabRight(_) => "/input/GrabRight".to_string(),
            VRChatOSCInput::DropLeft(_) => "/input/DropLeft".to_string(),
            VRChatOSCInput::UseLeft(_) => "/input/UseLeft".to_string(),
            VRChatOSCInput::GrabLeft(_) => "/input/GrabLeft".to_string(),
            VRChatOSCInput::PanicButton(_) => "/input/PanicButton".to_string(),
            VRChatOSCInput::QuickMenuToggleLeft(_) => "/input/QuickMenuToggleLeft".to_string(),
            VRChatOSCInput::QuickMenuToggleRight(_) => "/input/QuickMenuToggleRight".to_string(),
            VRChatOSCInput::Voice(_) => "/input/Voice".to_string(),

            VRChatOSCInput::ChatboxInput(_, _) => "/chatbox/input".to_string(),
            VRChatOSCInput::ChatboxTyping(_) => "/chatbox/typing".to_string(),

            VRChatOSCInput::Avatar(addr, _) => format!("/avatar/parameters/{}", addr.trim_matches('/')),
            VRChatOSCInput::Custom(addr, _) => addr.to_string(),
        }
    }
}
