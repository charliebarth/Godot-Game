use godot::obj::WithBaseField;

use crate::player::enums::player_events::PlayerEvents;
use crate::player::{
    enums::player_states::PlayerStates, player::Player, traits::player_state::PlayerState,
};

const CROUCH_SPEED: f32 = 75.0;

#[derive(Clone, Copy)]
pub struct Crouch;

impl PlayerState for Crouch {
    fn enter(_player: &mut Player) {}

    fn update(player: &mut Player) {
        let mut input_manager_unbound = player.get_input_manager();
        let mut input_manager = input_manager_unbound.bind_mut();

        if input_manager.fetch_player_event(PlayerEvents::Jump) && player.jump_available() {
            player.set_state(PlayerStates::Jump);
        } else if input_manager.fetch_player_event(PlayerEvents::Crouch) {
            player.set_state(PlayerStates::CrouchEnd);
        } else if !player.base().is_on_floor() {
            player.set_state(PlayerStates::Fall);
        } else if input_manager.fetch_player_event(PlayerEvents::Roll) {
            if player.get_horizontal_movement() != 0.0 {
                player.set_state(PlayerStates::Roll);
            } else {
                player.set_state(PlayerStates::CrouchEnd);
            }
        } else if input_manager.fetch_player_event(PlayerEvents::Sprint) {
            player.set_state(PlayerStates::Run);
        } else {
            Crouch::run(player);
        }
    }
}

impl Crouch {
    fn run(player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        player.set_dir(horizontal_dir);
        player.apply_horizontal_velocity(horizontal_dir, CROUCH_SPEED);

        let animation_speed = if horizontal_dir == 0.0 || horizontal_dir.abs() > 0.5 {
            horizontal_dir.abs()
        } else {
            0.5
        };

        player.set_animation_speed(animation_speed);
    }
}
