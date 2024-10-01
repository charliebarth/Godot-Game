use godot::global::godot_print;

use crate::player::enums::metal_events::{BurnType, MetalEvents};
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

pub struct Pewter {
    capacity: f64,
    current_reserve: f64,
    burn_rate: f64,
    low_burn_rate: f64,
}

impl Pewter {
    pub fn new(capacity: f64, current_reserve: f64, burn_rate: f64, low_burn_rate: f64) -> Self {
        Self {
            capacity,
            current_reserve,
            burn_rate,
            low_burn_rate,
        }
    }
}

impl Metal for Pewter {
    fn burn(&mut self, player: &mut Player) {
        self.current_reserve -= self.burn_rate;
        player.set_run_speed(player.get_run_speed() * 1.5);
        player.set_jump_force(player.get_jump_force() * 1.5);
    }

    fn low_burn(&mut self, player: &mut Player) {
        self.current_reserve -= self.low_burn_rate;
        player.set_run_speed(player.get_run_speed() * 1.2);
        player.set_jump_force(player.get_jump_force() * 1.2);
    }

    fn update(&mut self, player: &mut Player) {
        let mut godot_input_manager = player.get_input_manager();
        let mut input_manager = godot_input_manager.bind_mut();

        if self.current_reserve <= 0.0 {
            return;
        }

        if input_manager.fetch_metal_event(MetalEvents::Pewter(BurnType::Burn)) {
            self.burn(player);
        } else if input_manager.fetch_metal_event(MetalEvents::Pewter(BurnType::LowBurn)) {
            self.low_burn(player);
        }

        let mut metal_reserve_bar_manager_godot = player.get_metal_reserve_bar_manager();
        let mut metal_reserve_bar_manager = metal_reserve_bar_manager_godot.bind_mut();
        metal_reserve_bar_manager.adjust_bar_amount("pewter", self.current_reserve);
    }
}
