use godot::{builtin::Vector2, obj::WithBaseField};

use crate::player::{player::Player, traits::player_state::PlayerState};

use super::{fall::Fall, land::Land};

const MAX_JUMP_HEIGHT: f32 = 375.0;
const MAX_MOVE_SPEED: f32 = 100.0;

#[derive(Clone)]
pub struct Jump;

impl PlayerState for Jump {
    fn enter(&self, player: &mut Player) {
        let dir = player.get_horizontal_movement();
        player.apply_horizontal_velocity(dir, 125.0);

        let mut base = player.base_mut();
        if !base.is_on_floor() {
            return;
        }

        let jump_force = base.get_velocity().y + -MAX_JUMP_HEIGHT;
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

        player.set_dir(horizontal_dir);
        player.apply_horizontal_velocity(horizontal_dir, MAX_MOVE_SPEED);

        // TODO: Rewrite this and as_str to swap between run and walk
        let animation_speed = if horizontal_dir.abs() < 0.3 {
            0.3
        } else {
            horizontal_dir.abs()
        };

        player.get_sprite().set_speed_scale(animation_speed);
    }
}
