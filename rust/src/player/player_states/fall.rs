//! fall.rs
//! This file contains the implementation of the Fall state for the player.
//! The Fall state is responsible for handling the player's fall animation
//! and transitioning to other states based on player input and conditions.
//!
//! Author: Charles Barth, Michael Imerman
//! Version: Spring 2025
use godot::obj::WithBaseField;

use crate::player::{
    enums::{
        player_events::PlayerEvents, player_states::PlayerStates, timeout_events::TimeoutEvents,
    },
    player::Player,
    traits::player_state::PlayerState,
};

/// The default gravity for the player when falling
const FALL_GRAVITY: f64 = 1500.0;

#[derive(Clone, Copy)]
pub struct Fall;

impl PlayerState for Fall {
    fn enter(player: &mut Player) {
        if player.get_previous_state() != PlayerStates::Jump {
            player.add_timeout_event(TimeoutEvents::CoyoteTime);
        }
    }

    fn update(player: &mut Player) {
        let mut input_manager_unbound = player.get_input_manager();
        let mut input_manager = input_manager_unbound.bind_mut();

        if player.base().is_on_floor() {
            player.set_state(PlayerStates::Land);
        } else if input_manager.check_for_player_event(PlayerEvents::Jump)
            && player.jump_available()
        {
            player.set_state(PlayerStates::Jump);
        } else if input_manager.fetch_player_event(PlayerEvents::Attack) {
            player.set_state(PlayerStates::Attack);
        } else {
            Fall::run(player);
            Fall::fall(player);
        }
    }
}

impl Fall {
    /// Applies horizontal velocity to the player so they can move while falling
    ///
    /// # Arguments
    /// * `player` - The player object that is falling
    fn run(player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            return;
        }

        let speed = if horizontal_dir.signum() == player.get_dir().signum() {
            player.get_run_speed()
        } else {
            player.get_run_speed()
        };

        player.set_dir(horizontal_dir);
        player.apply_horizontal_velocity(horizontal_dir, speed);
    }

    /// This is used to swap from the jump gravity to the fall gravity
    /// when the player's vertical velocity in no longer upwards
    ///
    /// # Arguments
    /// * `player` - The player
    fn fall(player: &mut Player) {
        const FALL_GRAVITY_MULTIPLIER: f64 = 1.6;
        let vertical_velocity = player.base().get_velocity().y;

        // If the player is actually falling then multiply the default gravity by a preset multiplier
        // This is done so the player can jump satisfying heights while still returning to the ground quickly,
        // resulting in the controls for the player feeling more responive
        if vertical_velocity >= 0.0 {
            let gravity = player.get_default_gravity() * FALL_GRAVITY_MULTIPLIER;
            player.set_gravity(gravity);
        }
    }
}
