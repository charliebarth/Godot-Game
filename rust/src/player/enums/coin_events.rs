use std::fmt::{self, Display};

#[derive(PartialEq)]
pub enum CoinState {
    Idle, 
    PickedUp, 
    Thrown,
}

impl fmt::Display for CoinState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CoinState::Idle => write!(f, "Idle"),
            CoinState::PickedUp => write!(f, "Picked Up"),
            CoinState::Thrown => write!(f, "Thrown"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum CoinEvents {
    Throw,
    Drop,
}

impl CoinEvents {
    /// Converts from a string to a corresponding coin event. 
    pub fn from_string(button: &str) -> Option<CoinEvents> {
        match button {
            "throw" => Some(CoinEvents::Throw),
            "drop" => Some(CoinEvents::Drop),
            _ => None,
        }
    }
}