use godot::obj::WithBaseField;

use crate::player::{
    enums::{
        player_events::PlayerEvents, player_states::PlayerStates, timeout_events::TimeoutEvents,
    },
    player::Player,
    traits::player_state::PlayerState,
};

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
        if player.get_is_steel_burning() {
            return;
        }

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

    fn fall(player: &mut Player) {
        let vertical_velocity = player.base().get_velocity().y;
        if vertical_velocity >= 0.0 {
            player.set_gravity(FALL_GRAVITY);
        }
    }
}
