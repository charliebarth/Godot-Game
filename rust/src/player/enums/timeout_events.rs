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
}
