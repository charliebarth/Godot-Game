use crate::player::player::Player;

pub trait PlayerState {
    fn enter(&self, player: &mut Player);
    fn update(&self, player: &mut Player);
    fn clone(&self) -> Box<dyn PlayerState>;
}
