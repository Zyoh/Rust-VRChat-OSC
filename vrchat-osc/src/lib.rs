// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

mod input;
mod engine;
mod output;
mod value;

pub use input::Input;
pub use output::Output;
pub use engine::Engine;
pub use value::Value;

#[cfg(test)]
mod tests {
    use super::*;

    use input::Input;
    use engine::Engine;

    #[test]
    fn test_send_voice_value() {
        let engine = Engine::default();
        let message = Input::Voice(false);
        engine.send(message).unwrap();
    }
}
