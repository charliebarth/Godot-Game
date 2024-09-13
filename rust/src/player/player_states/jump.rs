use godot::{builtin::Vector2, obj::WithBaseField};

use crate::player::{player::Player, traits::player_state::PlayerState};

use super::{fall::Fall, idle::Idle, jump_fall::JumpFall, land::Land};

const MAX_JUMP_HEIGHT: f32 = 375.0;

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
        let y_vel = player.base_mut().get_velocity().y;

        if y_vel > -10.0 && y_vel < 10.0 {
            player.set_state(Box::new(JumpFall))
        } else if player.base_mut().is_on_floor() {
            player.set_state(Box::new(Land))
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Jump)
    }

    fn as_str(&self) -> &str {
        "jump"
    }
}
