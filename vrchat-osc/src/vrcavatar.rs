// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

pub enum VRChatOSCOutput {
    IsLocal(bool),
    Viseme(u8),
    Voice(f32),
    GestureLeft(u8),
    GestureRight(u8),
    GestureLeftWeight(f32),
    GestureRightWeight(f32),
    AngularY(f32),
    VelocityX(f32),
    VelocityY(f32),
    VelocityZ(f32),
    Upright(f32),
    Grounded(bool),
    Seated(bool),
    AFK(bool),
    TrackingType(u8),
    VRMode(u8),
    MuteSelf(bool),
    InStation(bool),
    Earmuff(bool),

    Int(String, u8),
    Float(String, f32),
    Bool(String, bool),
}

// TODO: Implement cast from OscMessage to VRChatOSCOutput
