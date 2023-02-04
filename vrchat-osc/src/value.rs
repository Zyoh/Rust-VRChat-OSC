// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use rosc::OscType;

pub(crate) fn constrain_float(value: f32) -> f32 {
    if value > 1.0 {
        1.0
    } else if value < -1.0 {
        -1.0
    } else {
        value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Float(f32),
    Int(u8),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float(constrain_float(value))
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::Int(value)
    }
}

impl TryInto<bool> for Value {
    type Error = &'static str;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Self::Bool(value) => Ok(value),
            _ => Err("Value is not bool."),
        }
    }
}

impl TryInto<f32> for Value {
    type Error = &'static str;

    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Self::Float(value) => Ok(value),
            _ => Err("Value is not float."),
        }
    }
}

impl TryInto<u8> for Value {
    type Error = &'static str;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Self::Int(value) => Ok(value),
            _ => Err("Value is not int."),
        }
    }
}

impl Into<OscType> for Value {
    fn into(self) -> OscType {
        match self {
            Value::Bool(value) => OscType::Bool(value),
            Value::Float(value) => OscType::Float(value),
            Value::Int(value) => OscType::Int(value as i32),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_constrain_float() {
        assert_eq!(constrain_float(0.0), 0.0);
        assert_eq!(constrain_float(1.0), 1.0);
        assert_eq!(constrain_float(-1.0), -1.0);
        assert_eq!(constrain_float(1.1), 1.0);
        assert_eq!(constrain_float(-1.1), -1.0);
    }

    #[test]
    fn test_value_from_bool() {
        assert_eq!(Value::from(true), Value::Bool(true));
        assert_eq!(Value::from(false), Value::Bool(false));
    }

    #[test]
    fn test_value_from_float() {
        assert_eq!(Value::from(0.0), Value::Float(0.0));
        assert_eq!(Value::from(1.0), Value::Float(1.0));
        assert_eq!(Value::from(-1.0), Value::Float(-1.0));
        assert_eq!(Value::from(1.1), Value::Float(1.0));
        assert_eq!(Value::from(-1.1), Value::Float(-1.0));
    }

    #[test]
    fn test_value_from_int() {
        assert_eq!(Value::from(0), Value::Int(0));
        assert_eq!(Value::from(1), Value::Int(1));
        assert_eq!(Value::from(255), Value::Int(255));
    }
}
