use godot::{builtin::StringName, classes::Input, obj::WithBaseField};

use crate::player::{player::Player, traits::player_state::PlayerState};

use super::{fall::Fall, idle::Idle, jump::Jump, run::Run};

#[derive(Clone)]
pub struct Land;

impl PlayerState for Land {
    fn enter(&self, _player: &mut Player) {}

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
        } else {
            player.set_state(Box::new(Idle));
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Land)
    }
}
