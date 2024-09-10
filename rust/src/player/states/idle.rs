use crate::player::traits::player_state::State;

#[derive(PartialEq, Debug)]
pub struct Idle {}

impl State for Idle {
    fn enter() {}

    fn update() {}
}
