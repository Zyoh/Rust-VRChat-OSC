// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

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

            VRChatOSCInput::Custom(_, value) => value.to_owned(),
        }
    }

    pub fn to_osc_addr(&self) -> String {
        match self {
            VRChatOSCInput::Vertical(_) => "/input/Vertical",
            VRChatOSCInput::Horizontal(_) => "/input/Horizontal",
            VRChatOSCInput::LookHorizontal(_) => "/input/LookHorizontal",
            VRChatOSCInput::UseAxisRight(_) => "/input/UseAxisRight",
            VRChatOSCInput::GrabAxisRight(_) => "/input/GrabAxisRight",
            VRChatOSCInput::MoveHoldFB(_) => "/input/MoveHoldFB",
            VRChatOSCInput::SpinHoldCwCcw(_) => "/input/SpinHoldCwCcw",
            VRChatOSCInput::SpinHoldUD(_) => "/input/SpinHoldUD",
            VRChatOSCInput::SpinHoldLR(_) => "/input/SpinHoldLR",

            VRChatOSCInput::MoveForward(_) => "/input/MoveForward",
            VRChatOSCInput::MoveBackward(_) => "/input/MoveBackward",
            VRChatOSCInput::MoveLeft(_) => "/input/MoveLeft",
            VRChatOSCInput::MoveRight(_) => "/input/MoveRight",
            VRChatOSCInput::LookLeft(_) => "/input/LookLeft",
            VRChatOSCInput::LookRight(_) => "/input/LookRight",
            VRChatOSCInput::Jump(_) => "/input/Jump",
            VRChatOSCInput::Run(_) => "/input/Run",
            VRChatOSCInput::ComfortLeft(_) => "/input/ComfortLeft",
            VRChatOSCInput::ComfortRight(_) => "/input/ComfortRight",
            VRChatOSCInput::DropRight(_) => "/input/DropRight",
            VRChatOSCInput::UseRight(_) => "/input/UseRight",
            VRChatOSCInput::GrabRight(_) => "/input/GrabRight",
            VRChatOSCInput::DropLeft(_) => "/input/DropLeft",
            VRChatOSCInput::UseLeft(_) => "/input/UseLeft",
            VRChatOSCInput::GrabLeft(_) => "/input/GrabLeft",
            VRChatOSCInput::PanicButton(_) => "/input/PanicButton",
            VRChatOSCInput::QuickMenuToggleLeft(_) => "/input/QuickMenuToggleLeft",
            VRChatOSCInput::QuickMenuToggleRight(_) => "/input/QuickMenuToggleRight",
            VRChatOSCInput::Voice(_) => "/input/Voice",

            VRChatOSCInput::ChatboxInput(_, _) => "/chatbox/input",
            VRChatOSCInput::ChatboxTyping(_) => "/chatbox/typing",

            VRChatOSCInput::Custom(addr, _) => addr,
        }.to_string()
    }
}
