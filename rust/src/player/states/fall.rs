use crate::player::traits::state::PlayerState;

#[derive(PartialEq, Debug)]
pub struct Fall {}

impl PlayerState for Fall {
    fn enter() {}

    fn update() {}
}
