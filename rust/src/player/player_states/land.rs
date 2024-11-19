use godot::obj::WithBaseField;

use crate::player::{
    enums::player_events::PlayerEvents, enums::player_states::PlayerStates, player::Player,
    traits::player_state::PlayerState,
};

#[derive(Clone, Copy)]
pub struct Land;

impl PlayerState for Land {
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
        } else {
            player.set_state(PlayerStates::Idle);
        }
    }
}
