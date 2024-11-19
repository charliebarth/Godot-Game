use godot::obj::WithBaseField;

use crate::player::{
    enums::{force::Force, player_events::PlayerEvents, player_states::PlayerStates},
    player::Player,
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
        let mut next_state: PlayerStates = PlayerStates::Run;

        if horizontal_dir == 0.0 {
            next_state = PlayerStates::Idle;
        } else if input_manager.check_for_player_event(PlayerEvents::Jump) && player.jump_available() {
            next_state = PlayerStates::Jump;
        } else if !player.base().is_on_floor() {
            next_state = PlayerStates::Fall;
        } else if input_manager.fetch_player_event(PlayerEvents::Crouch) {
            next_state = PlayerStates::CrouchStart;
        } else if input_manager.fetch_player_event(PlayerEvents::Roll) {
            next_state = PlayerStates::Roll;
        } else if input_manager.fetch_player_event(PlayerEvents::Sprint) {
            next_state = PlayerStates::Sprint;
        } else if input_manager.fetch_player_event(PlayerEvents::Attack) {
            next_state = PlayerStates::Attack;
        }

        if next_state != PlayerStates::Run {
            Run::exit(player, next_state);
        } else {
            Run::run(player);
        }
    }
}

impl Run {
    fn run(player: &mut Player) {
        let run_strength = player.get_horizontal_movement();

        if run_strength.signum() != player.get_dir().signum() {
            player.add_force(Force::Run { acceleration: 0.0 });
        }
        player.set_dir(run_strength);

        let scaled_speed = player.get_min_run_speed()
            + run_strength.abs() * (player.get_run_speed() - player.get_min_run_speed());

        player.set_run_speed(scaled_speed);

        // This is the acceleration of the player
        // Make this a constant or field of the player
        let speed = 900.0;
        player.add_force(Force::Run {
            acceleration: run_strength * speed,
        });

        // TODO: Chagne this to be based on the actual speed of the player
        let animation_speed = if run_strength.abs() < 0.25 {
            0.25
        } else {
            run_strength.abs()
        };

        player.set_animation_speed(animation_speed);
    }

    fn exit(player: &mut Player, next_state: PlayerStates) {
        player.set_state(next_state);
    }
}
