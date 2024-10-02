use godot::{builtin::Vector2, obj::WithBaseField};

use crate::player::{player::Player, traits::player_state::PlayerState};

use super::{fall::Fall, land::Land};

// TODO: Allow the player to flip direction in the first couple of frames of the jump
// TODO: Only reduce the backwards momentum if the signum of the horizontal velocity is opposite.
// If the players momentum is in the same direction or zero, then don't reduce it.

#[derive(Clone)]
pub struct Jump;

impl PlayerState for Jump {
    fn enter(&self, player: &mut Player) {
        let jump_force = player.get_jump_force();
        let mut base = player.base_mut();

        let jump_force = base.get_velocity().y + -jump_force;
        let jump_vel = Vector2::new(base.get_velocity().x, base.get_velocity().y + jump_force);
        base.set_velocity(jump_vel);
    }

    fn update(&self, player: &mut Player) {
        if player.is_anim_finished() {
            player.set_state(Box::new(Fall))
        } else if player.base_mut().is_on_floor() {
            player.set_state(Box::new(Land))
        } else {
            self.run(player);
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Jump)
    }

    fn as_str(&self, player: &mut Player) -> &str {
        let y_vel = player.base_mut().get_velocity().y;
        if y_vel > -10.0 {
            "jump_fall"
        } else {
            "jump"
        }
    }
}

impl Jump {
    fn run(&self, player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            return;
        }

        if horizontal_dir.signum() != player.get_dir().signum() {
            player.apply_horizontal_velocity(horizontal_dir, player.get_run_speed() / 2.0);
        } else {
            player.apply_horizontal_velocity(horizontal_dir, player.get_run_speed());
        }
    }
}
