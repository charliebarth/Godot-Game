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
        // TODO: If not on the floor set the state to fall (push would interupt a jump)
        self.current_reserve -= self.burn_rate;
        // TODO: Make constant
        let max_acceleration: f32 = 6000.0;

        let player_position = player.base().get_global_position();
        let metal_position = player.get_metal_object_position(0);

        // TODO: Remove
        if metal_position.x == 0.0 && metal_position.y == 0.0 {
            return;
        }

        if !player.base().is_on_floor() {
            player.add_force(Force::NormalForce { magnitude: -1.0 });
        }

        let angle = atan2(
            player_position.y as f64 - metal_position.y as f64,
            player_position.x as f64 - metal_position.x as f64,
        );

        let x_acceleration: f32 = max_acceleration * (angle.cos() as f32);
        let y_acceleration: f32 = max_acceleration * (angle.sin() as f32);

        player.add_force(Force::SteelPush {
            x_acceleration,
            y_acceleration,
        });
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
