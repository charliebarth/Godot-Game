use godot::obj::WithBaseField;

use crate::player::{
    enums::{force::Force, player_states::PlayerStates},
    player::Player,
    traits::player_state::PlayerState,
};

// TODO: Allow the player to flip direction in the first couple of frames of the jump
// TODO: Only reduce the backwards momentum if the signum of the horizontal velocity is opposite.
// If the players momentum is in the same direction or zero, then don't reduce it.

const JUMP_GRAVITY: f64 = 980.0;

#[derive(Clone, Copy)]
pub struct Jump;

impl PlayerState for Jump {
    fn enter(player: &mut Player) {
        player.set_gravity(JUMP_GRAVITY);

        let jump_force = player.get_jump_force();
        player.add_force(Force::Jump {
            velocity: -jump_force,
        });
    }

    fn update(player: &mut Player) {
        let next_state: PlayerStates;

        if player.is_anim_finished() {
            next_state = PlayerStates::Fall;
        } else if player.base().is_on_floor() {
            next_state = PlayerStates::Land;
        } else {
            next_state = PlayerStates::Jump;
        }

        if next_state != PlayerStates::Jump {
            Jump::exit(player, next_state);
        } else {
            Jump::run(player);
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
}
