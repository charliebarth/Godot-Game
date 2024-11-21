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
            PlayerEvents::LowBurn => TriggerEvents::OnPress,
            PlayerEvents::Die => TriggerEvents::OnPress,
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
    /// This is a modifier key on the controller.
    /// If button is held/the event is registered and a metal event is triggered, the metal event will be triggered with the LowBurn variant.
    LowBurn,
    Die,
}

impl PlayerEvents {
    /// Converts a string to the corresponding player event.
    pub fn from_string(button: &str) -> Option<PlayerEvents> {
        match button {
            "jump" => Some(PlayerEvents::Jump),
            "sprint" => Some(PlayerEvents::Sprint),
            "roll" => Some(PlayerEvents::Roll),
            "attack" => Some(PlayerEvents::Attack),
            "low_burn" => Some(PlayerEvents::LowBurn),
            "die" => Some(PlayerEvents::Die),
            _ => None,
        }
    }

    /// This determines the number of frames a player event should be held in a vec of registered events before expiring and being removed.
    /// -1 means the event should persist until the button is released
    pub fn timeout(&self) -> i8 {
        match self {
            PlayerEvents::Jump => 12,
            PlayerEvents::Crouch => 10,
            PlayerEvents::Roll => 10,
            PlayerEvents::Sprint => 10,
            PlayerEvents::Attack => 10,
            PlayerEvents::LowBurn => -1,
            PlayerEvents::Die => 10,
        }
    }
}
