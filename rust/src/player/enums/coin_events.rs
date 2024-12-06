/// Holds the CoinState and CoinEvents Enums
/// 
/// Author: Trinity Pittman
/// Date: Fall 2024
use std::fmt::{self, Display};

#[derive(PartialEq)]
/// This enum is used to describe the states the coin is in 
pub enum CoinState {
    Idle,       // The coin is in idle state (can be picked up)
    PickedUp,   // The coin is in picked up state (can be thrown)
    Thrown,     // The thrown is in thrown state (can damage)
}

/// Implement display for Displaying coin states 
impl fmt::Display for CoinState {

    /// Given a coin state, gives a string representation 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CoinState::Idle => write!(f, "Idle"),
            CoinState::PickedUp => write!(f, "Picked Up"),
            CoinState::Thrown => write!(f, "Thrown"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
/// Represents Coin events that could happen, most notably the player pressing throw 
pub enum CoinEvents {
    Throw,  // When the player presses throw
    Drop,   // Unused for potential drop 
}

/// Methods that belong to the CoinEvents enum 
impl CoinEvents {

    /// Converts from a string to a corresponding coin event. 
    pub fn from_string(button: &str) -> Option<CoinEvents> {
        match button {
            "throw" => Some(CoinEvents::Throw),
            "drop" => Some(CoinEvents::Drop), // Unused 
            _ => None,
        }
    }
}