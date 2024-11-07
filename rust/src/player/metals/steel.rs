use godot::global::{atan2, godot_print};
use godot::obj::WithBaseField;

use crate::player::enums::force::Force;
use crate::player::enums::metal_events::{BurnType, MetalEvents};
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

pub struct Steel {
    capacity: f64,
    current_reserve: f64,
    burn_rate: f64,
    low_burn_rate: f64,
}

impl Steel {
    pub fn new(capacity: f64, current_reserve: f64, burn_rate: f64, low_burn_rate: f64) -> Self {
        Self {
            capacity,
            current_reserve,
            burn_rate,
            low_burn_rate,
        }
    }
}

impl Metal for Steel {
    fn burn(&mut self, player: &mut Player) {
        self.current_reserve -= self.burn_rate;
        // TODO: Push
        let max_acceleration = 4000.0;
        player.add_force(Force::SteelPush {
            acceleration_x: max_acceleration / 2.0,
            acceleration_y: -(max_acceleration / 1.8),
        });
        let position = player.base().get_global_position();
        let health_bar = player.get_health_bar().get_global_position();
        godot_print!("Y: {}", position.y as f64 - health_bar.y as f64);
        let angle = atan2(
            position.y as f64 - health_bar.y as f64,
            position.x as f64 - health_bar.x as f64,
        );
        godot_print!("Angle: {}", angle.to_degrees());
    }

    fn low_burn(&mut self, player: &mut Player) {
        self.current_reserve -= self.low_burn_rate;
        // TODO: Show allomantic lines
    }

    fn update(&mut self, player: &mut Player) {
        let mut godot_input_manager = player.get_input_manager();
        let mut input_manager = godot_input_manager.bind_mut();

        if self.current_reserve <= 0.0 {
            return;
        }

        // Burning is an actual steel push
        if input_manager.fetch_metal_event(MetalEvents::Steel(BurnType::Burn)) {
            self.burn(player);
        }

        // Low burning shows the allomantic lines
        if input_manager.fetch_metal_event(MetalEvents::Steel(BurnType::LowBurn)) {
            self.low_burn(player);
        }

        let mut metal_reserve_bar_manager_godot = player.get_metal_reserve_bar_manager();
        let mut metal_reserve_bar_manager = metal_reserve_bar_manager_godot.bind_mut();
        metal_reserve_bar_manager.adjust_bar_amount("steel", self.current_reserve);
    }
}
