use godot::{builtin::StringName, classes::Input, obj::WithBaseField};

use crate::player::{player::Player, traits::player_state::PlayerState};

use super::{fall::Fall, idle::Idle, jump::Jump, turn_around::TurnAround};

#[derive(Clone)]
pub struct Run;

impl PlayerState for Run {
    fn enter(&self, player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            return;
        }

        player.set_dir(horizontal_dir);
        player.apply_horizontal_velocity(horizontal_dir, 125.0);
    }

    fn update(&self, player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            // TODO: Replace with decelerate state
            player.set_state(Box::new(Idle));
        } else if Input::singleton().is_action_just_pressed(StringName::from("jump"))
            && player.base().is_on_floor()
        {
            player.set_state(Box::new(Jump));
        } else if !player.base().is_on_floor() {
            player.set_state(Box::new(Fall));
        } else if horizontal_dir != player.get_dir() {
            player.set_state(Box::new(TurnAround));
        } else if horizontal_dir == player.get_dir() {
            player.set_state(Box::new(Run));
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Run)
    }
}
