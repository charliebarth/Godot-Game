use crate::player::traits::state::PlayerState;

#[derive(PartialEq, Debug)]
pub struct Run {}

impl PlayerState for Run {
    fn enter() {}

    fn update() {}
}
