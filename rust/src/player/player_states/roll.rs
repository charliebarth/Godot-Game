use godot::obj::WithBaseField;

use crate::player::{player::Player, player_states::land::Land, traits::player_state::PlayerState};

#[derive(Clone)]
pub struct Roll;

impl PlayerState for Roll {
    fn enter(&self, _player: &mut Player) {}

    fn update(&self, player: &mut Player) {
        if player.base().is_on_floor() {
            player.set_state(Box::new(Land));
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Roll)
    }

    fn as_str(&self, player: &mut Player) -> &str {
        "fall"
    }
}
