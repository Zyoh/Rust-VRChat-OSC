// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

pub mod vrcinput;
pub mod vrcosc;

fn contrain_value(value: f32) -> f32 {
    if value > 1.0 {
        1.0
    } else if value < -1.0 {
        -1.0
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use vrcinput::VRChatOSCInput;
    use vrcosc::VRChatOSC;

    #[test]
    fn test_send_voice_value() {
        let vrchat_osc = VRChatOSC::default();
        let message = VRChatOSCInput::Voice(false);
        vrchat_osc.send_vrc_input(message).unwrap();
    }
}
