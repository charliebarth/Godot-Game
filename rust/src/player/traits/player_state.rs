//! player_state.rs
//! This file defines the PlayerState trait, which is used to define the different states a player
//! can be in. The PlayerState trait defines the functions that a player state must implement.
//!
//! Author: Charles Barth
//! Version: Spring 2025

use crate::player::player::Player;

/// This trait defines the functions that a player state must implement.
pub trait PlayerState {
    /// This function is called when the player enters the state.
    /// It is responsible for any one time logic that should occur in the state.
    /// It does not check for any state transitions.
    ///
    /// # Arguments
    /// * `player` - The player owner of the state.
    fn enter(player: &mut Player);

    /// This function is called every frame the player is in the state.
    /// It is responsible for updating the player's state and checking for state transitions.
    ///
    /// # Arguments
    /// * `player` - The player owner of the state.
    fn update(player: &mut Player);
}
