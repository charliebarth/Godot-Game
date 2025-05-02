//! land.rs
//!
//! This file contains the implementation of the Land state for the player.
//! The Land state is responsible for handling the player's landing animation
//! and transitioning to other states based on player input and conditions.
//!
//! Author: Charles Barth
//! Version: Spring 2025
use godot::{classes::AnimatedSprite2D, obj::WithBaseField};

use crate::player::{
    enums::{player_events::PlayerEvents, player_states::PlayerStates},
    player::Player,
    traits::player_state::PlayerState,
};

#[derive(Clone, Copy)]
pub struct Land;

impl PlayerState for Land {
    fn enter(player: &mut Player) {
        let mut dust = player.base().get_node_as::<AnimatedSprite2D>("Dust");
        dust.set_visible(true);
    }

    fn update(player: &mut Player) {
        let mut input_manager_unbound = player.get_input_manager();
        let input_manager = input_manager_unbound.bind_mut();
        let horizontal_movement = input_manager.get_left_right_value();

        if input_manager.check_for_player_event(PlayerEvents::Jump) && player.jump_available() {
            player.set_state(PlayerStates::Jump);
        } else if !player.base().is_on_floor() {
            player.set_state(PlayerStates::Fall);
        } else if horizontal_movement != 0.0 {
            player.set_state(PlayerStates::Run);
        } else {
            player.set_state(PlayerStates::Idle);
        }
    }
}
