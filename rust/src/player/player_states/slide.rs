// slide.rs
//
// This file contains the implementation of the Slide state for the player.
// The Slide state is responsible for handling the player's slide animation
// and transitioning to other states based on player input and conditions.
//
// Author: Charles Barth
// Version: Spring 2025
use crate::player::{player::Player, traits::player_state::PlayerState};

#[derive(Clone, Copy)]
pub struct Slide;

impl PlayerState for Slide {
    fn enter(player: &mut Player) {
        let dir = player.get_dir();
        let speed = player.get_run_speed() * 1.5;
        player.apply_horizontal_velocity(dir, speed);
    }

    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            let previous_state = player.get_previous_state();
            player.set_state(previous_state);
        }
    }
}
