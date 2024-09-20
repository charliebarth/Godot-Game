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
            _ => None,
        }
    }
}
