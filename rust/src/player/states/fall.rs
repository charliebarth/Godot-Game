use crate::player::traits::player_state::State;

#[derive(PartialEq, Debug)]
pub struct Fall {}

impl State for Fall {
    fn enter() {}

    fn update() {}
}
