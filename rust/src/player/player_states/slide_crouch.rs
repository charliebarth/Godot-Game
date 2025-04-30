// slide_crouch.rs
//
// This file contains the implementation of the SlideCrouch state for the player.
// The SlideCrouch state is responsible for handling the player's slide crouch animation
// and transitioning to other states based on player input and conditions.
//
// Author: Charles Barth
// Version: Spring 2025
use crate::player::{
    enums::player_states::PlayerStates, player::Player, traits::player_state::PlayerState,
};

#[derive(Clone, Copy)]
pub struct SlideCrouch;

impl PlayerState for SlideCrouch {
    fn enter(_player: &mut Player) {}

    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            player.set_state(PlayerStates::Crouch);
        }
    }
}
