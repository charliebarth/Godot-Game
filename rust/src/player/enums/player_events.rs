#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerEvents {
    Jump,
    Crouch,
    Roll,
}

impl PlayerEvents {
    // Method to convert from string to the corresponding event
    pub fn from_string(button: &str) -> Option<PlayerEvents> {
        match button {
            "A" => Some(PlayerEvents::Jump),
            _ => None,
        }
    }
}
