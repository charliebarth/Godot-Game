use crate::player::player::Player;

pub trait Metal {
    fn update(&self, player: &mut Player);
    fn burn(&self, player: &mut Player);
    fn low_burn(&self, player: &mut Player);
}
