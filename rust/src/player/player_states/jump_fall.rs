use godot::obj::WithBaseField;

use crate::player::{player::Player, traits::player_state::PlayerState};

use super::{fall::Fall, land::Land};

#[derive(Clone)]
pub struct JumpFall;

impl PlayerState for JumpFall {
    fn enter(&self, _player: &mut Player) {}

    fn update(&self, player: &mut Player) {
        if player.is_anim_finished() {
            player.set_state(Box::new(Fall));
        } else if player.base().is_on_floor() {
            player.set_state(Box::new(Land));
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(JumpFall)
    }
}
