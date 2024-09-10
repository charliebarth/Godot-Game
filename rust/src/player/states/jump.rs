use crate::player::traits::state::State;

#[derive(PartialEq, Debug)]
pub struct Jump {}

impl State for Jump {
    fn enter() {}

    fn update() {}
}
