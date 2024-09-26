use crate::player::player::Player;

pub trait Metal {
    fn update(&mut self, player: &mut Player);
    fn burn(&mut self, player: &mut Player);
    fn low_burn(&mut self, player: &mut Player);
}
