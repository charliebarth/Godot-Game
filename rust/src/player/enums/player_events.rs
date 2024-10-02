#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum TriggerEvents {
    OnPress,
    OnRelease,
}

impl TriggerEvents {
    pub fn trigger_for_player_event(event: PlayerEvents) -> TriggerEvents {
        match event {
            PlayerEvents::Jump => TriggerEvents::OnPress,
            PlayerEvents::Crouch => TriggerEvents::OnRelease,
            PlayerEvents::Roll => TriggerEvents::OnRelease,
            PlayerEvents::Sprint => TriggerEvents::OnPress,
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerEvents {
    Jump,
    Crouch,
    Roll,
    Sprint,
}

impl PlayerEvents {
    // Method to convert from string to the corresponding event
    pub fn from_string(button: &str) -> Option<PlayerEvents> {
        match button {
            "jump" => Some(PlayerEvents::Jump),
            "sprint" => Some(PlayerEvents::Sprint),
            "roll" => Some(PlayerEvents::Roll),
            _ => None,
        }
    }
}
