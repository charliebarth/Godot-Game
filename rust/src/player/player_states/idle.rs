use crate::player::{player::Player, traits::player_state::PlayerState};

#[derive(Clone)]
pub struct Idle;

impl PlayerState for Idle {
    fn enter(&self, player: &mut Player) {}

    fn update(&self, player: &mut Player) {}

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Idle)
    }
}
