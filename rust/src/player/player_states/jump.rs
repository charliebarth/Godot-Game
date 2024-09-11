use crate::player::{player::Player, traits::player_state::PlayerState};

use super::idle::Idle;

#[derive(Clone)]
pub struct Jump;

impl PlayerState for Jump {
    fn enter(&self, player: &mut Player) {}

    fn update(&self, player: &mut Player) {
        player.set_state(Box::new(Idle))
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Jump)
    }
}
