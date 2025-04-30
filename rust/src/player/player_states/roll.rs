// roll.rs
//
// This file contains the implementation of the Roll state for the player.
// The Roll state is responsible for handling the player's roll animation
// and transitioning to other states based on player input and conditions.
//
// Author: Charles Barth
use crate::player::{player::Player, traits::player_state::PlayerState};

#[derive(Clone, Copy)]
pub struct Roll;

impl PlayerState for Roll {
    fn enter(player: &mut Player) {
        player.set_animation_speed(1.0);
    }

    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            let previous_state = player.get_previous_state();
            player.set_state(previous_state);
        }
    }
}
