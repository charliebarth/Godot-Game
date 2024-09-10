use crate::player::traits::state::PlayerState;

#[derive(PartialEq, Debug)]
pub struct Idle {}

impl PlayerState for Idle {
    fn enter() {}

    fn update() {}
}
