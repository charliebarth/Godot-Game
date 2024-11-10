use godot::builtin::{Color, Vector2};
use godot::global::{atan2, godot_print};
use godot::obj::WithBaseField;

use crate::player::enums::force::Force;
use crate::player::enums::metal_events::{BurnType, MetalEvents};
use crate::player::enums::player_states::PlayerStates;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

pub struct Steel {
    capacity: f64,
    current_reserve: f64,
    burn_rate: f64,
    low_burn_rate: f64,
    was_low_burn: bool,
}

impl Steel {
    pub fn new(capacity: f64, current_reserve: f64, burn_rate: f64, low_burn_rate: f64) -> Self {
        Self {
            capacity,
            current_reserve,
            burn_rate,
            low_burn_rate,
            was_low_burn: false,
        }
    }
}

impl Metal for Steel {
    fn burn(&mut self, player: &mut Player) {
        if player.get_metal_objects().is_empty() {
            return;
        }

        player.set_is_steel_burning(true);
        player.set_state(PlayerStates::Fall);
        // TODO: If not on the floor set the state to fall (push would interupt a jump)
        self.current_reserve -= self.burn_rate;
        // TODO: Make constant
        let max_acceleration: f32 = 6000.0;

        let player_position = player.base().get_global_position();
        let metal_position = Vector2::new(0.0, 0.0);

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
        self.was_low_burn = true;
        self.current_reserve -= self.low_burn_rate;

        let mut metal_line = player.get_metal_line();
        let mut bound_metal_line = metal_line.bind_mut();
        bound_metal_line.set_should_show(true);

        let metal_objects = player.get_metal_objects();
        //let player_mass = player.get_mass();

        for metal_object in metal_objects.iter() {
            let color = Color::from_rgba(0.117647, 0.564706, 1.0, 0.7);
            let metal_object_position = metal_object.get_global_position();
            //let metal_object_mass = metal_object.get_mass();

            bound_metal_line.add_line_segment(metal_object_position, color);
        }

        drop(bound_metal_line);

        metal_line.queue_redraw();
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
        } else {
            player.set_is_steel_burning(false);
        }

        // Low burning shows the allomantic lines
        if input_manager.fetch_metal_event(MetalEvents::Steel(BurnType::LowBurn)) {
            self.low_burn(player);
        } else if self.was_low_burn {
            self.was_low_burn = false;

            let mut metal_line = player.get_metal_line();
            let mut bound_metal_line = metal_line.bind_mut();
            bound_metal_line.set_should_show(false);
            drop(bound_metal_line);

            metal_line.queue_redraw();
        }

        let mut metal_reserve_bar_manager_godot = player.get_metal_reserve_bar_manager();
        let mut metal_reserve_bar_manager = metal_reserve_bar_manager_godot.bind_mut();
        metal_reserve_bar_manager.adjust_bar_amount("steel", self.current_reserve);
    }
}
