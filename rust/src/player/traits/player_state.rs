use crate::player::player::Player;

pub trait PlayerState {
    fn enter(player: &mut Player);
    fn update(player: &mut Player);
}
