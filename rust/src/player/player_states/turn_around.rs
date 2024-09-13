use godot::{builtin::StringName, classes::Input, obj::WithBaseField};

use crate::player::{player::Player, traits::player_state::PlayerState};

use super::{fall::Fall, idle::Idle, jump::Jump, run::Run};

#[derive(Clone)]
pub struct TurnAround;

impl PlayerState for TurnAround {
    fn enter(&self, player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            return; // No movement
        }

        let mut base_vel = player.base_mut().get_velocity();

        // If player has already turned, accelerate in the new direction
        if horizontal_dir != player.get_dir() {
            // Start acceleration from the current velocity, increasing until reaching normal speed
            if base_vel.x.abs() < 125.0 {
                base_vel.x += horizontal_dir * 5.0; // Gradually increase the speed
            }
        }

        player.base_mut().set_velocity(base_vel);
    }

    fn update(&self, player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        if Input::singleton().is_action_just_pressed(StringName::from("jump"))
            && player.base().is_on_floor()
        {
            player.set_state(Box::new(Jump));
        } else if !player.base().is_on_floor() {
            player.set_state(Box::new(Fall));
        } else if horizontal_dir == 0.0 {
            player.set_state(Box::new(Idle));
        } else if player.is_anim_finished() {
            player.set_state(Box::new(Run))
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(TurnAround)
    }

    fn as_str(&self) -> &str {
        "turn_around"
    }
}
