use godot::obj::WithBaseField;

use crate::player::{
    enums::player_events::PlayerEvents, enums::player_states::PlayerStates, player::Player,
    traits::player_state::PlayerState,
};

#[derive(Clone, Copy)]
pub struct Sprint;

impl PlayerState for Sprint {
    fn enter(player: &mut Player) {
        Sprint::run(player);
    }

    fn update(player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();
        let mut input_manager_unbound = player.get_input_manager();
        let mut input_manager = input_manager_unbound.bind_mut();

        if horizontal_dir.signum() != player.get_dir().signum() || horizontal_dir == 0.0 {
            player.set_state(PlayerStates::Idle);
        } else if input_manager.fetch_player_event(PlayerEvents::Jump) && player.jump_available() {
            player.set_state(PlayerStates::Jump);
        } else if !player.base().is_on_floor() {
            player.set_state(PlayerStates::Fall);
        // If player attempts to crouch while sprinting they slide into a crouch
        } else if input_manager.fetch_player_event(PlayerEvents::Crouch) {
            player.set_state(PlayerStates::SlideCrouch);
        // If player attempts to roll while sprinting they slide instead
        } else if input_manager.fetch_player_event(PlayerEvents::Roll) {
            player.set_state(PlayerStates::Slide);
        } else {
            Sprint::run(player);
        }
    }
}

impl Sprint {
    fn run(player: &mut Player) {
        let dir = player.get_dir();
        let speed = player.get_run_speed() * 1.3;
        player.apply_horizontal_velocity(dir, speed);

        player.get_sprite().set_speed_scale(1.3);
    }
}
