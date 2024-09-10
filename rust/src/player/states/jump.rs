use crate::player::traits::player_state::PlayerState;

#[derive(PartialEq, Debug)]
pub struct Jump {}

impl PlayerState for Jump {
    fn enter() {}

    fn update() {}
}
