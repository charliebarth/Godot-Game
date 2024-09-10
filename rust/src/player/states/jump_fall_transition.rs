use crate::player::traits::state::PlayerState;

#[derive(PartialEq, Debug)]
pub struct JumpFallTransition {}

impl PlayerState for JumpFallTransition {
    fn enter() {}

    fn update() {}
}
