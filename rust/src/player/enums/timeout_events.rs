//! timeout_events.rs
//!
//! This file defines the `TimeoutEvents` enum and its associated methods.
//!
//! Author: Charles Barth
//! Version: Spring 2025
use std::time::Duration;

/// An enum of events that are available for only a specific duration until they time out.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TimeoutEvents {
    /// A brief period of time where the player can jump after walking off a ledge
    CoyoteTime,
}

impl TimeoutEvents {
    /// Returns the duration of the timeout event.
    ///
    /// # Returns
    /// * `Duration` - The duration of the timeout event.
    pub fn get_duration(&self) -> Duration {
        match self {
            TimeoutEvents::CoyoteTime => Duration::from_millis(100),
        }
    }

    /// Serializes a timeout event to a string.
    ///
    /// # Returns
    /// * `String` - The serialized timeout event.
    pub fn serialize(&self) -> String {
        match self {
            TimeoutEvents::CoyoteTime => "coyote_time".to_string(),
        }
    }

    /// Deserializes a timeout event from a string.
    ///
    /// # Arguments
    /// * `s` - The string to deserialize the timeout event from
    ///
    /// # Returns
    /// * `Option<TimeoutEvents>` - The deserialized timeout event.
    pub fn deserialize(s: &str) -> Option<TimeoutEvents> {
        match s {
            "coyote_time" => Some(TimeoutEvents::CoyoteTime),
            _ => None,
        }
    }
}
