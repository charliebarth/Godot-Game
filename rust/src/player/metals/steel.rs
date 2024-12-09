use godot::builtin::Color;
use godot::obj::WithBaseField;

use crate::player::enums::force::Force;
use crate::player::enums::metal_events::MetalEvents;
use crate::player::enums::player_states::PlayerStates;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

pub struct Steel {
    capacity: f64,
    current_reserve: f64,
    burn_rate: f64,
    low_burn_rate: f64,
    was_low_burn: bool,
    /// -1.0 when its a push, 1.0 when its a pull, and 0.0 when its not being used
    push: f32,
    show_particles: bool,
}

impl Steel {
    pub fn new(capacity: f64, current_reserve: f64, burn_rate: f64, low_burn_rate: f64) -> Self {
        Self {
            capacity,
            current_reserve,
            burn_rate,
            low_burn_rate,
            was_low_burn: false,
            push: 0.0,
            show_particles: false,
        }
    }
}

impl Metal for Steel {
    fn burn(&mut self, player: &mut Player) {
        if !self.was_low_burn {
            return;
        }

        let metal_position = player.get_nearest_metal_object();

        if metal_position.is_none() {
            return;
        }

        player.set_is_steel_burning(true);
        player.set_state(PlayerStates::Fall);
        self.current_reserve -= self.burn_rate;
        // TODO: Make constant
        let max_acceleration: f32 = 700.0;

        if !player.base().is_on_floor() {
            player.add_force(Force::NormalForce { magnitude: -1.0 });
        }

        if let Some(angle) = metal_position {
            let x_velocity: f32 = max_acceleration * (angle.cos() as f32) * self.push;
            let y_velocity: f32 = max_acceleration * (angle.sin() as f32) * self.push;

            player.add_force(Force::SteelPush {
                x_velocity,
                y_velocity,
            });
        }
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
            let color = Color::from_rgba(0.0, 0.6, 2.3, 1.0);
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
        self.show_particles = false;
        self.push = 0.0;

        // Burning is an actual steel push
        if self.current_reserve > 0.0 {
            if input_manager.fetch_metal_event(MetalEvents::Steel) {
                self.push += -1.0;
            }

            if input_manager.fetch_metal_event(MetalEvents::Iron) {
                self.push += 1.0;
            };

            if self.push != 0.0 {
                self.burn(player)
            }
        }

        if self.current_reserve <= 0.0 || self.push == 0.0 {
            player.set_is_steel_burning(false);
        }

        // Low burning shows the allomantic lines
        if self.current_reserve > 0.0 && input_manager.fetch_metal_event(MetalEvents::SteelLowBurn)
        {
            self.low_burn(player);
            self.show_particles = true;
        } else if self.was_low_burn {
            self.was_low_burn = false;

            let mut metal_line = player.get_metal_line();
            let mut bound_metal_line = metal_line.bind_mut();
            bound_metal_line.set_should_show(false);
            drop(bound_metal_line);

            metal_line.queue_redraw();
        }

        player.set_metal_reserve_amount(self.as_str().into(), self.current_reserve);
        player.set_metal_reserve_amount("iron".into(), self.current_reserve);
        player
            .get_steel_particles()
            .set_visible(self.show_particles);
    }

    fn as_str(&self) -> &str {
        "steel"
    }

    fn increase_reserve(&mut self, amount: f64) {
        self.current_reserve += amount;
        self.current_reserve = self.current_reserve.clamp(0.0, self.capacity);
    }
}
