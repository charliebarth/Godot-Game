use crate::player::traits::player_state::State;

#[derive(PartialEq, Debug)]
pub struct Jump {}

impl State for Jump {
    fn enter() {}

    fn update() {}
}
