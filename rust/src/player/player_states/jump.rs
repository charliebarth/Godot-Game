use godot::{builtin::Vector2, global::godot_print, obj::WithBaseField};

use crate::player::{
    enums::player_states::PlayerStates, player::Player, traits::player_state::PlayerState,
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
        let mut base = player.base_mut();

        // let jump_force = base.get_velocity().y + -jump_force;
        let jump_vel = Vector2::new(base.get_velocity().x, -jump_force);
        base.set_velocity(jump_vel);
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
        let horizontal_dir = player.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            return;
        }

        let run_speed = player.get_run_speed();

        player.set_dir(horizontal_dir);
        player.apply_horizontal_velocity(horizontal_dir, run_speed);
    }

    fn exit(player: &mut Player, next_state: PlayerStates) {
        player.set_state(next_state);
    }
}
