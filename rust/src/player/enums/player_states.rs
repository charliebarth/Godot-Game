#[derive(Debug, Hash, PartialEq, Eq, Clone)]

/// States that the player can be in
pub enum PlayerStates {
    Crouch,
    CrouchEnd,
    CrouchStart,
    Fall,
    Idle,
    Jump,
    Land,
    Roll,
    Run,
    Slide,
    Sprint,
    WallSlide,
}

impl PlayerStates {
    pub fn as_str(&self) -> &str {
        match self {
            PlayerStates::Crouch => "crouch",
            PlayerStates::CrouchEnd => "crouch_end",
            PlayerStates::CrouchStart => "crouch_start",
            PlayerStates::Fall => "fall",
            PlayerStates::Idle => "idle",
            PlayerStates::Jump => "jump",
            PlayerStates::Land => "land",
            PlayerStates::Roll => "roll",
            PlayerStates::Run => "run",
            PlayerStates::Slide => "slide",
            PlayerStates::Sprint => "sprint",
            PlayerStates::WallSlide => "wall_slide",
        }
    }
}
