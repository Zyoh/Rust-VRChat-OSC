// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use rosc::{OscMessage, OscType};

macro_rules! output {
    (
        u8 {
            $($name_int:ident,)*
        }

        f32 {
            $($name_float:ident,)*
        }

        bool {
            $($name_bool:ident,)*
        }
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Output {
            $( $name_int(u8), )*
            $( $name_float(f32), )*
            $( $name_bool(bool), )*

            Int(String, u8),
            Float(String, f32),
            Bool(String, bool),
        }

        impl TryFrom<OscMessage> for Output {
            type Error = &'static str;

            fn try_from(message: OscMessage) -> Result<Self, Self::Error> {
                let addr = message.addr.as_str();
                let addr_stem = addr.replace("/avatar/parameters/", "");
                let args = message.args;

                if args.len() != 1 {
                    return Err("Invalid number of arguments.");
                }

                match args[0] {
                    OscType::Int(value) => {
                        match addr_stem.as_str() {
                            $( stringify!($name_int) => Ok(Output::$name_int(value as u8)), )*
                            _ => Err("Invalid argument value.")
                        }
                    },
                    OscType::Float(value) => {
                        match addr_stem.as_str() {
                            $( stringify!($name_float) => Ok(Output::$name_float(value)), )*
                            _ => Err("Invalid argument value.")
                        }
                    },
                    OscType::Bool(value) => {
                        match addr_stem.as_str() {
                            $( stringify!($name_bool) => Ok(Output::$name_bool(value)), )*
                            _ => Err("Invalid argument value.")
                        }
                    },
                    _ => Err("Invalid argument type."),
                }
            }
        } 
    }
}

output! {
    u8 {
        Viseme,
        GestureLeft,
        GestureRight,
        TrackingType,
        VRMode,
    }

    f32 {
        GestureLeftWeight,
        GestureRightWeight,
        AngularY,
        VelocityX,
        VelocityY,
        VelocityZ,
        Upright,
        Voice,
    }

    bool {
        IsLocal,
        Grounded,
        Seated,
        AFK,
        MuteSelf,
        InStation,
        Earmuff,
    }    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_output() {
        let message = OscMessage {
            addr: "/avatar/parameters/Viseme".to_string(),
            args: vec![OscType::Int(1)],
        };

        let output = Output::try_from(message).unwrap();

        assert_eq!(output, Output::Viseme(1));
    }
}
