use std::time::Duration;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TimeoutEvents {
    CoyoteTime,
}

impl TimeoutEvents {
    pub fn get_duration(&self) -> Duration {
        match self {
            TimeoutEvents::CoyoteTime => Duration::from_millis(100),
        }
    }
}
