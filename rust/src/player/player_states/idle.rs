use godot::obj::WithBaseField;

use crate::player::{
    enums::{force::Force, player_events::PlayerEvents, player_states::PlayerStates},
    player::Player,
    traits::player_state::PlayerState,
};

#[derive(Clone, Copy)]
pub struct Idle;

impl PlayerState for Idle {
    fn enter(_player: &mut Player) {}

    fn update(player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();
        let mut input_manager_unbound = player.get_input_manager();
        let mut input_manager = input_manager_unbound.bind_mut();

        if input_manager.check_for_player_event(PlayerEvents::Jump) && player.jump_available() {
            player.set_state(PlayerStates::Jump);
        } else if !player.base().is_on_floor() {
            player.set_state(PlayerStates::Fall);
        } else if horizontal_dir != 0.0 {
            player.set_state(PlayerStates::Run);
        } else if input_manager.fetch_player_event(PlayerEvents::Crouch) {
            player.set_state(PlayerStates::CrouchStart);
        } else if input_manager.fetch_player_event(PlayerEvents::Roll) {
            player.set_state(PlayerStates::CrouchStart);
        } else if input_manager.fetch_player_event(PlayerEvents::Attack) {
            player.set_state(PlayerStates::Attack);
        } else {
            player.add_force(Force::Stop {
                horizontal: true,
                vertical: false, // TODO: This should also maybe be true
            });
        }
    }
}
