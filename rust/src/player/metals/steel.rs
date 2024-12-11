use godot::builtin::Color;
use godot::obj::WithBaseField;

use crate::player::enums::force::Force;
use crate::player::enums::metal_events::MetalEvents;
use crate::player::enums::player_states::PlayerStates;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

/// The steel player ability.
/// This ability allows the player to push and pull on metal objects.
/// If the player pushes on a metal object then the player will be pushed in the opposite direction.
/// If the player pulls on a metal object then the player will be pulled in the direction of the metal object.
/// This done by calculating the angle between the player and the metal object and then applying a percentage of the max acceleration
/// based on how far off the angle is from either 0, 90, 180, or 270 degrees.
pub struct Steel {
    /// The maximum amount of steel the player can store.
    capacity: f64,
    /// The current amount of steel the player has.
    current_reserve: f64,
    /// The rate at which the player burns steel.
    burn_rate: f64,
    /// The rate at which the player burns steel when using the low burn ability.
    low_burn_rate: f64,
    /// A flag to determine if the player was low burning.
    was_low_burn: bool,
    /// The push value for the steel ability.
    /// -1.0 when its a push, 1.0 when its a pull, and 0.0 when its not being used
    push: f32,
    /// A flag to determine if the player should show the steel particles.
    show_particles: bool,
}

impl Steel {
    /// The constructor for the steel struct.
    ///
    /// # Arguments
    /// * `capacity` - The maximum amount of steel the player can store.
    /// * `current_reserve` - The current amount of steel the player has.
    /// * `burn_rate` - The rate at which the player burns steel.
    /// * `low_burn_rate` - The rate at which the player burns steel when using the low burn ability.
    ///
    /// # Returns
    /// * `Steel` - A new instance of the steel struct.
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
    /// The burn function for steel.
    /// This function pushes or pulls the player towards or away from the metal object nearest to the line selector node.
    /// The player will be pushed or pulled based on the angle between the player and the metal object.
    /// The angle is used to calculate the x and y velocity of the player.
    /// A percentage of the max acceleration is then applied to the player based on how far off the angle is from either 0, 90, 180, or 270 degrees.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the force can be modified.
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

    /// The low burn function for steel.
    /// This function shows the allomantic lines for the metal objects.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the metal line can be modified.
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

    /// The update function for steel.
    /// This function check to see if the input manager has a steel event.
    /// If the event is found then the burn function is called.
    /// If the low burn variant is found then the low burn function is called.
    /// This will also toggle the steel particles on and off.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the input manager can be accessed.
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
