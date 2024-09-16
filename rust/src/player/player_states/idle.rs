use godot::{builtin::StringName, classes::Input, obj::WithBaseField};

use crate::player::{player::Player, traits::player_state::PlayerState};

use super::{fall::Fall, jump::Jump, run::Run};

#[derive(Clone)]
pub struct Idle;

impl PlayerState for Idle {
    fn enter(&self, player: &mut Player) {
        player.apply_horizontal_velocity(1.0, 0.0);
    }

    fn update(&self, player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        if Input::singleton().is_action_just_pressed(StringName::from("jump"))
            && player.base().is_on_floor()
        {
            player.set_state(Box::new(Jump));
        } else if !player.base().is_on_floor() {
            player.set_state(Box::new(Fall));
        } else if horizontal_dir != 0.0 {
            player.set_state(Box::new(Run));
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Idle)
    }

    fn as_str(&self, _player: &mut Player) -> &str {
        "idle"
    }
}
