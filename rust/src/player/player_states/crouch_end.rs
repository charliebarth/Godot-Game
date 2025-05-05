//! crouch_end.rs
//!
//! This file contains the implementation of the CrouchEnd state for the player.
//! The CrouchEnd state is responsible for handling the player's crouch end animation.
//!
//! Author: Charles Barth
//! Version: Spring 2025
use crate::player::{
    enums::player_states::PlayerStates, player::Player, traits::player_state::PlayerState,
};

// NOTE: This and most of the other states have not had documentation added
// because Rust will automatically derive the documentation from the PlayerState trait.
// The functions in these traits work nearly identically in each state and just have slightly different conditions
// for swapping to a new state.

#[derive(Clone, Copy)]
pub struct CrouchEnd;

impl PlayerState for CrouchEnd {
    /// This function is called when the player enters the CrouchEnd state.
    /// It is responsible for any one time logic that should occur in the state.
    /// It does not check for any state transitions.
    ///
    /// # Arguments
    /// * `player` - The player owner of the state.
    fn enter(_player: &mut Player) {}

    /// This function is called every frame the player is in the CrouchEnd state.
    /// It is responsible for updating the player's state and checking for state transitions.
    ///
    /// # Arguments
    /// * `player` - The player owner of the state.
    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            player.set_state(PlayerStates::Idle);
        }
    }
}
