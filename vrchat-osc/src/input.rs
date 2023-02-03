// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use crate::value::{Value, constrain_float};

use rosc::{OscType, OscMessage};

macro_rules! input {
    (
        f32 {
            $($name_float:ident,)*
        }

        bool {
            $($name_bool:ident,)*
        }
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Input {
            $( $name_float(f32), )*
            $( $name_bool(bool), )*

            ChatboxInput(String, bool),
            ChatboxTyping(bool),

            Avatar(String, Value),
            Custom(String, Vec<OscType>),
        }

        impl Into<OscMessage> for Input {
            fn into(self) -> OscMessage {
                let address = match &self {
                    $( Self::$name_float(_) => format!("/input/{}", stringify!($name_float)), )*
                    $( Self::$name_bool(_) => format!("/input/{}", stringify!($name_bool)), )*

                    Self::ChatboxInput(_, _) => "/chatbox/input".to_owned(),
                    Self::ChatboxTyping(_) => "/chatbox/typing ".to_owned(),

                    Self::Avatar(addr, _) => {
                        let avatar_param_addr = addr.strip_prefix('/').unwrap_or(addr);
                        format!("/avatar/parameters/{}", avatar_param_addr)
                    },
                    Self::Custom(addr, _) => addr.to_owned(),
                };

                let values = match &self {
                    $( Self::$name_float(value) => vec![OscType::Float(constrain_float(*value))], )*
                    $( Self::$name_bool(value) => vec![OscType::Bool(*value)], )*

                    Self::ChatboxInput(value, immediate) => vec![OscType::String(value.to_owned()), OscType::Bool(*immediate)],
                    Self::ChatboxTyping(value) => vec![OscType::Bool(*value)],

                    Self::Avatar(_, value) => vec![value.to_osc_type()],
                    Self::Custom(_, value) => value.to_owned(),
                };

                OscMessage {
                    addr: address,
                    args: values,
                }
            }
        }
    };
}

input! {
    f32 {
        Vertical,
        Horizontal,
        LookHorizontal,
        UseAxisRight,
        GrabAxisRight,
        MoveHoldFB,
        SpinHoldCwCcw,
        SpinHoldUD,
        SpinHoldLR,
    }
    
    bool {
        MoveForward,
        MoveBackward,
        MoveLeft,
        MoveRight,
        LookLeft,
        LookRight,
        Jump,
        Run,
        ComfortLeft,
        ComfortRight,
        DropRight,
        UseRight,
        GrabRight,
        DropLeft,
        UseLeft,
        GrabLeft,
        PanicButton,
        QuickMenuToggleLeft,
        QuickMenuToggleRight,
        Voice,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_address() {
        let message: OscMessage = Input::Vertical(0.0).into();
        assert_eq!(message.addr, "/input/Vertical");
    }

    #[test]
    fn test_float() {
        for (value, cvalue) in 
            vec![(-2.0, -1.0), (-1.0, -1.0), (-0.5, -0.5), (0.0, 0.0), (0.5, 0.5), (1.0, 1.0), (2.0, 1.0)] {
            
            let message: OscMessage = Input::Vertical(value).into();
            assert_eq!(message.args, vec![OscType::Float(cvalue)]);
        }
    }

    #[test]
    fn test_bool() {
        for (value, cvalue) in 
            vec![(false, false), (true, true)] {
            
            let message: OscMessage = Input::MoveForward(value).into();
            assert_eq!(message.args, vec![OscType::Bool(cvalue)]);
        }
    }
}
