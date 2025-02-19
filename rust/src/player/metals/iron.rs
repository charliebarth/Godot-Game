use godot::obj::Gd;

use crate::player::{
    enums::metal_type::{ButtonState, MetalType},
    input_manager::InputManager,
    player::Player,
    traits::metal::Metal,
};

use super::steel::Steel;

const PULL_BURN_DIRECTION: f32 = 1.0;

pub struct Iron {
    steel: Steel,
}

impl Iron {
    pub fn new(
        capacity: f64,
        current_reserve: f64,
        burn_rate: f64,
        low_burn_rate: f64,
        player: Gd<Player>,
        metal_type: MetalType,
    ) -> Self {
        let mut steel = Steel::new(
            capacity,
            current_reserve,
            burn_rate,
            low_burn_rate,
            player,
            metal_type,
        );
        steel.set_burn_direction(PULL_BURN_DIRECTION);
        Iron { steel }
    }
}

impl Metal for Iron {
    fn update(&mut self) {
        self.steel.update();
    }

    fn burn(&mut self) {
        self.steel.burn();
    }

    fn low_burn(&mut self) {
        self.steel.low_burn();
    }

    fn update_reserve(&mut self, amount: f64) {
        self.steel.update_reserve(amount);
    }

    fn metal_type(&self) -> MetalType {
        self.steel.metal_type()
    }

    fn update_low_burn(&mut self, input_manager: &mut Gd<InputManager>) {
        self.steel.update_low_burn(input_manager);
    }

    fn update_burn(&mut self, input_manager: &mut Gd<InputManager>) {
        self.steel.update_burn(input_manager);
    }
}
