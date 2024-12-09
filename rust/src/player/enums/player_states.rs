use crate::player::{
    player::Player,
    player_states::{
        attack::Attack, crouch::Crouch, crouch_end::CrouchEnd, crouch_start::CrouchStart,
        fall::Fall, idle::Idle, jump::Jump, land::Land, roll::Roll, run::Run, slide::Slide,
        slide_crouch::SlideCrouch, sprint::Sprint,
    },
    traits::player_state::PlayerState,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]

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
    SlideCrouch,
    Attack,
}

impl PlayerStates {
    pub fn as_str(&self) -> &str {
        match self {
            PlayerStates::Crouch => "crouch_walk",
            PlayerStates::CrouchEnd => "crouch_end",
            PlayerStates::CrouchStart => "crouch_start",
            PlayerStates::Fall => "fall",
            PlayerStates::Idle => "idle",
            PlayerStates::Jump => "jump",
            PlayerStates::Land => "land",
            PlayerStates::Roll => "roll",
            PlayerStates::Run => "run",
            PlayerStates::Slide => "slide",
            PlayerStates::Sprint => "run",
            PlayerStates::SlideCrouch => "slide",
            PlayerStates::Attack => "attack",
        }
    }

    pub fn update_state(self, player: &mut Player) {
        match self {
            PlayerStates::Idle => Idle::update(player),
            PlayerStates::Run => Run::update(player),
            PlayerStates::Jump => Jump::update(player),
            PlayerStates::Fall => Fall::update(player),
            PlayerStates::Land => Land::update(player),
            PlayerStates::Roll => Roll::update(player),
            PlayerStates::Crouch => Crouch::update(player),
            PlayerStates::CrouchEnd => CrouchEnd::update(player),
            PlayerStates::CrouchStart => CrouchStart::update(player),
            PlayerStates::Slide => Slide::update(player),
            PlayerStates::SlideCrouch => SlideCrouch::update(player),
            PlayerStates::Sprint => Sprint::update(player),
            PlayerStates::Attack => Attack::update(player),
        }
    }

    pub fn enter_state(self, player: &mut Player) {
        match self {
            PlayerStates::Idle => Idle::enter(player),
            PlayerStates::Run => Run::enter(player),
            PlayerStates::Jump => Jump::enter(player),
            PlayerStates::Fall => Fall::enter(player),
            PlayerStates::Land => Land::enter(player),
            PlayerStates::Roll => Roll::enter(player),
            PlayerStates::Crouch => Crouch::enter(player),
            PlayerStates::CrouchEnd => CrouchEnd::enter(player),
            PlayerStates::CrouchStart => CrouchStart::enter(player),
            PlayerStates::Slide => Slide::enter(player),
            PlayerStates::SlideCrouch => SlideCrouch::enter(player),
            PlayerStates::Sprint => Sprint::enter(player),
            PlayerStates::Attack => Attack::enter(player),
        }
    }
}
