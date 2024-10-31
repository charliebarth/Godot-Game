use godot::obj::WithBaseField;

use crate::player::{
    enums::player_events::PlayerEvents, enums::player_states::PlayerStates, player::Player,
    traits::player_state::PlayerState,
};

#[derive(Clone, Copy)]
pub struct Run;

impl PlayerState for Run {
    fn enter(player: &mut Player) {
        Run::run(player);
    }

    fn update(player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();
        let mut input_manager_unbound = player.get_input_manager();
        let mut input_manager = input_manager_unbound.bind_mut();

        if horizontal_dir == 0.0 {
            player.set_state(PlayerStates::Idle);
        } else if input_manager.fetch_player_event(PlayerEvents::Jump) && player.jump_available() {
            player.set_state(PlayerStates::Jump);
        } else if !player.base().is_on_floor() {
            player.set_state(PlayerStates::Fall);
        } else if input_manager.fetch_player_event(PlayerEvents::Crouch) {
            player.set_state(PlayerStates::CrouchStart);
        } else if input_manager.fetch_player_event(PlayerEvents::Roll) {
            player.set_state(PlayerStates::Roll);
        } else if input_manager.fetch_player_event(PlayerEvents::Sprint) {
            player.set_state(PlayerStates::Sprint);
        } else {
            Run::run(player);
        }
    }
}

impl Run {
    fn run(player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            return;
        }

        let speed = player.get_run_speed();
        player.set_dir(horizontal_dir);
        player.apply_horizontal_velocity(horizontal_dir, speed);

        let animation_speed = if horizontal_dir.abs() < 0.3 {
            0.3
        } else {
            horizontal_dir.abs()
        };

        player.set_animation_speed(animation_speed);
    }
}
