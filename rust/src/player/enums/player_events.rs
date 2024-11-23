#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
/// This enum is used to determine whether an event registered by the InputManager should be triggered on press or release.
pub enum TriggerEvents {
    /// The event should be triggered on press.
    OnPress,
    /// The event should be triggered on release.
    OnRelease,
}

impl TriggerEvents {
    /// Returns the corresponding trigger event for a given player event.
    pub fn trigger_for_player_event(event: PlayerEvents) -> TriggerEvents {
        match event {
            PlayerEvents::Jump => TriggerEvents::OnPress,
            PlayerEvents::Crouch => TriggerEvents::OnRelease,
            PlayerEvents::Roll => TriggerEvents::OnRelease,
            PlayerEvents::Sprint => TriggerEvents::OnPress,
            PlayerEvents::Attack => TriggerEvents::OnPress,
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
/// This enum defines the valid player events and is used to identify events in the InputManager.
pub enum PlayerEvents {
    /// The player has jumped.
    Jump,
    /// The player has crouched.
    Crouch,
    /// The player has rolled.
    Roll,
    /// The player has sprinted.
    Sprint,
    /// The player has attacked.
    Attack,
}

impl PlayerEvents {
    /// Converts a string to the corresponding player event.
    pub fn from_string(button: &str) -> Option<PlayerEvents> {
        match button {
            "jump" => Some(PlayerEvents::Jump),
            "sprint" => Some(PlayerEvents::Sprint),
            "roll" => Some(PlayerEvents::Roll),
            "attack" => Some(PlayerEvents::Attack),
            _ => None,
        }
    }

    pub fn timeout(&self) -> u32 {
        match self {
            PlayerEvents::Jump => 10,
            PlayerEvents::Crouch => 10,
            PlayerEvents::Roll => 10,
            PlayerEvents::Sprint => 10,
            PlayerEvents::Attack => 10,
        }
    }
}
