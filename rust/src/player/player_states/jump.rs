//! jump.rs
//! This file contains the implementation of the Jump state for the player.
//! The Jump state is responsible for handling the player's jump animation
//! and transitioning to other states based on player input and conditions.
//!
//! Author: Charles Barth, Michael Imerman
//! Version: Spring 2025
use godot::obj::{GdMut, WithBaseField};

use crate::player::{
    enums::{force::Force, player_events::PlayerEvents, player_states::PlayerStates},
    input_manager::InputManager,
    player::Player,
    traits::player_state::PlayerState,
};

#[derive(Clone, Copy)]
pub struct Jump;

impl PlayerState for Jump {
    fn enter(player: &mut Player) {
        let jump_force = player.get_jump_force() * 0.5;
        player.add_force(Force::Jump {
            acceleration: -jump_force,
        });
    }

    fn update(player: &mut Player) {
        let next_state: PlayerStates;
        let mut input_manager_unbound = player.get_input_manager();
        let mut input_manager = input_manager_unbound.bind_mut();

        if !input_manager.check_for_player_event(PlayerEvents::Jump) {
            next_state = PlayerStates::Fall;
        } else if player.base().is_on_floor() {
            next_state = PlayerStates::Land;
        } else if input_manager.fetch_player_event(PlayerEvents::Attack) {
            next_state = PlayerStates::Attack;
        } else {
            next_state = PlayerStates::Jump;
        }

        if next_state != PlayerStates::Jump {
            Jump::exit(player, next_state);
        } else {
            Jump::run(player);
            Jump::jump(player, input_manager);
        }
    }
}

impl Jump {
    /// Applies horizontal velocity to the player so they can move while jumping
    ///
    /// # Arguments
    /// * `player` - The player
    fn run(player: &mut Player) {
        let run_strength = player.get_horizontal_movement();

        if run_strength == 0.0 {
            return;
        }

        if run_strength.signum() != player.get_dir().signum() {
            player.add_force(Force::AirRun { acceleration: 0.0 });
        }
        player.set_dir(run_strength);

        let scaled_speed = player.get_min_run_speed()
            + run_strength.abs() * (player.get_run_speed() - player.get_min_run_speed());

        player.set_run_speed(scaled_speed);

        // This is the acceleration of the player
        // Make this a constant or field of the player
        let speed = 900.0;
        player.add_force(Force::AirRun {
            acceleration: run_strength * speed,
        });
    }

    /// Exits the jump state and enters the next state
    /// This is used so there are fewer exit points in the update function
    ///
    /// # Arguments
    /// * `player` - The player
    /// * `next_state` - The next state to enter
    fn exit(player: &mut Player, next_state: PlayerStates) {
        player.set_state(next_state);
    }

    /// This method allows the player to determine their jump height based on how long the jump button is held
    /// The longer the jump button is held, the higher the player will jump until the jump event is released or
    /// automatically expired after a predetermined amount of time
    ///
    /// # Arguments
    /// * `player` - The player
    /// * `input_manager` - This is used to check if the jump button is pressed
    fn jump(player: &mut Player, input_manager: GdMut<'_, InputManager>) {
        if !input_manager.check_for_player_event(PlayerEvents::Jump) {
            return;
        }

        let jump_force = player.get_jump_force() * 0.065;
        player.add_force(Force::Jump {
            acceleration: -jump_force,
        });
    }
}
