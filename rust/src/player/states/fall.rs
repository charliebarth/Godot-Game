use crate::player::traits::state::State;

#[derive(PartialEq, Debug)]
pub struct Fall {}

impl State for Fall {
    fn enter() {}

    fn update() {}
}
