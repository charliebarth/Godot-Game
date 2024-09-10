use crate::player::traits::state::State;

#[derive(PartialEq, Debug)]
pub struct Idle {}

impl State for Idle {
    fn enter() {}

    fn update() {}
}
