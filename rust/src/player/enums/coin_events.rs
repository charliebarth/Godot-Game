//! Holds the CoinState and CoinEvents Enums
//!
//! Author: Trinity Pittman
//! Version: Fall 2024

use std::fmt::{self};

#[derive(PartialEq)]
#[derive(Debug)]
/// This enum is used to describe the states the coin is in
pub enum CoinState {
    /// The coin is in idle state (can be picked up)
    Idle,     
    /// The coin is in picked up state (can be thrown)
    PickedUp, 
    /// The thrown is in thrown state (can damage)
    Thrown,   
}

/// Implement display for Displaying coin states
impl fmt::Display for CoinState {
    /// Given a coin state, gives a string representation
    /// 
    /// # Arguments 
    /// * `f` (fmt::Formatter) - formatter 
    /// # Returns 
    /// * the result of the formatting 
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
    /// When the player presses throw
    Throw, 
    /// Unused for potential drop
    Drop,  
}

/// Methods that belong to the CoinEvents enum
impl CoinEvents {
    /// Converts from a string to a corresponding coin event.
    ///
    /// # Arguments
    /// * `button` - The string to convert to a coin event.
    ///
    /// # Returns
    /// * `Some(CoinEvents)` - The corresponding coin event.
    pub fn from_string(button: &str) -> Option<CoinEvents> {
        match button {
            "throw" => Some(CoinEvents::Throw),
            "drop" => Some(CoinEvents::Drop), // Unused
            _ => None,
        }
    }
}
