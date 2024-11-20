use godot::obj::WithBaseField;

use crate::player::{
    enums::{force::Force, player_events::PlayerEvents, player_states::PlayerStates},
    player::Player,
    traits::player_state::PlayerState,
};

const JUMP_GRAVITY: f64 = 980.0;

#[derive(Clone, Copy)]
pub struct Jump;

impl PlayerState for Jump {
    fn enter(player: &mut Player) {
        player.set_gravity(JUMP_GRAVITY);

        let jump_force = player.get_jump_force() * 0.5;
        player.add_force(Force::Jump {
            acceleration: -jump_force,
        });
    }

    fn update(player: &mut Player) {
        let next_state: PlayerStates;
        let mut input_manager_unbound = player.get_input_manager();
        let mut input_manager = input_manager_unbound.bind_mut();

        if player.is_anim_finished() {
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
            Jump::jump(player);
        }
    }
}

impl Jump {
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

    fn exit(player: &mut Player, next_state: PlayerStates) {
        player.set_state(next_state);
    }

    fn jump(player: &mut Player) {
        let input_manager = player.get_input_manager();
        let bound_input_manager = input_manager.bind();

        if !bound_input_manager.check_for_player_event(PlayerEvents::Jump) {
            return;
        }

        let jump_force = player.get_jump_force() * 0.065;
        player.add_force(Force::Jump {
            acceleration: -jump_force,
        });
    }
}
