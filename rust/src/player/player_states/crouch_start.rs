use crate::player::{player::Player, traits::player_state::PlayerState};

use super::crouch::Crouch;

#[derive(Clone)]
pub struct CrouchStart;

impl PlayerState for CrouchStart {
    fn enter(&self, _player: &mut Player) {}

    fn update(&self, player: &mut Player) {
        if player.is_anim_finished() {
            player.set_state(Box::new(Crouch));
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(CrouchStart)
    }

    fn as_str(&self, _player: &mut Player) -> &str {
        "crouch_start"
    }
}
