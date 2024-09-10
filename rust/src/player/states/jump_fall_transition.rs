use crate::player::traits::player_state::State;

#[derive(PartialEq, Debug)]
pub struct JumpFallTransition {}

impl State for JumpFallTransition {
    fn enter() {}

    fn update() {}
}
