use crate::player::traits::state::State;

#[derive(PartialEq, Debug)]
pub struct Run {}

impl State for Run {
    fn enter() {}

    fn update() {}
}
