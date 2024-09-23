use crate::player::player::Player;
use crate::player::traits::metal::Metal;

pub struct Pewter {
    capacity: f64,
    current_reserve: f64,
    burn_rate: f64,
    low_burn_rate: f64,
    is_low_burn: bool,
}

impl Pewter {
    pub fn new(capacity: f64, current_reserve: f64, burn_rate: f64, low_burn_rate: f64) -> Self {
        Self {
            capacity,
            current_reserve,
            burn_rate,
            low_burn_rate,
            is_low_burn: false,
        }
    }
}

impl Metal for Pewter {
    fn burn(&self, player: &mut Player) {}
    fn low_burn(&self, player: &mut Player) {}
    fn update(&self, player: &mut Player) {}
}
