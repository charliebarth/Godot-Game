//! player_states.rs
//!
//! This file defines the `PlayerStates` enum and its associated methods.
//!
//! Author: Charles Barth, Michael Imerman
//! Version: Spring 2025
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

/// Methods for the PlayerStates
impl PlayerStates {
    /// Converts the player state to a string.
    /// This is used to get the name of the animation to play.
    ///
    /// # Returns
    /// * `&str` - The name of the animation to play.
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

    /// Calls the update function for the current state.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the state can be updated.
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

    /// Calls the enter function for the current state.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the state can be updated.
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

    pub fn serialize(&self) -> String {
        match self {
            PlayerStates::Attack => "attack".to_string(),
            PlayerStates::Crouch => "crouch_walk".to_string(),
            PlayerStates::CrouchEnd => "crouch_end".to_string(),
            PlayerStates::CrouchStart => "crouch_start".to_string(),
            PlayerStates::Fall => "fall".to_string(),
            PlayerStates::Idle => "idle".to_string(),
            PlayerStates::Jump => "jump".to_string(),
            PlayerStates::Land => "land".to_string(),
            PlayerStates::Roll => "roll".to_string(),
            PlayerStates::Run => "run".to_string(),
            PlayerStates::Slide => "slide".to_string(),
            PlayerStates::SlideCrouch => "slide".to_string(),
            PlayerStates::Sprint => "run".to_string(),
        }
    }

    pub fn deserialize(s: &str) -> Option<PlayerStates> {
        match s {
            "attack" => Some(PlayerStates::Attack),
            "crouch_walk" => Some(PlayerStates::Crouch),
            "crouch_end" => Some(PlayerStates::CrouchEnd),
            "crouch_start" => Some(PlayerStates::CrouchStart),
            "fall" => Some(PlayerStates::Fall),
            "idle" => Some(PlayerStates::Idle),
            "jump" => Some(PlayerStates::Jump),
            "land" => Some(PlayerStates::Land),
            "roll" => Some(PlayerStates::Roll),
            "run" => Some(PlayerStates::Run),
            "slide" => Some(PlayerStates::Slide),
            _ => None,
        }
    }
}
