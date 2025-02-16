use godot::builtin::{Color, Vector2};
use godot::classes::Input;
use godot::global::JoyAxis;
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
    /// The location of the metal object that the player is currently pushing or pulling.
    object_location: Vector2,
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
            object_location: Vector2::ZERO,
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

        // Get the normalized direction from the player to the nearest metal object.
        if self.object_location == Vector2::ZERO {
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

        // Use the x and y components directly since 'direction' is normalized.
        let x_velocity = max_acceleration * self.object_location.x * self.push;
        let y_velocity = max_acceleration * self.object_location.y * self.push;

        player.add_force(Force::SteelPush {
            x_velocity,
            y_velocity,
        });
    }

    /// The low burn function for steel.
    /// This function shows the allomantic lines for the metal objects.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the metal line can be modified.
    fn low_burn(&mut self, player: &mut Player) {
        // Mark that the player is low burning and decrease the reserve.
        self.was_low_burn = true;
        self.current_reserve -= self.low_burn_rate;
        self.object_location = Vector2::ZERO;

        // Get the metal line and show it.
        let mut metal_line = player.get_metal_line();
        let player_position = metal_line.to_local(metal_line.get_global_position());
        let mut bound_metal_line = metal_line.bind_mut();
        bound_metal_line.set_should_show(true);

        // Get the joystick position.
        let joy_position_x =
            Input::singleton().get_joy_axis(player.get_device_id(), JoyAxis::RIGHT_X);
        let joy_position_y =
            Input::singleton().get_joy_axis(player.get_device_id(), JoyAxis::RIGHT_Y);
        let joy_position = Vector2::new(joy_position_x, joy_position_y);

        // A metal object must be within ±25 degrees to be selected.
        let mut closest_obj_angle_diff: f32 = 40.0_f32.to_radians();

        // Get the player position and the index of the closest metal object.
        let mut index_closest_metal_object = usize::MAX;

        for (index, metal_object) in player.get_metal_objects().iter().enumerate() {
            let color = Color::from_rgba(0.0, 0.4, 0.9, 0.1);
            let metal_object_position = bound_metal_line
                .base()
                .to_local(metal_object.get_global_position());

            bound_metal_line.add_line_segment(metal_object_position, color);

            let closest_metal_object = self.get_closest_metal_object(
                player_position,
                joy_position,
                closest_obj_angle_diff,
                metal_object_position,
            );

            if let Some((metal_object_dir, angle_diff)) = closest_metal_object {
                self.object_location = metal_object_dir;
                closest_obj_angle_diff = angle_diff;
                index_closest_metal_object = index;
            }
        }

        if index_closest_metal_object != usize::MAX {
            bound_metal_line.update_color(
                Color::from_rgba(0.0, 0.6, 2.3, 1.0),
                index_closest_metal_object,
            );
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

        // Low burning shows the allomantic lines
        if self.current_reserve > 0.0 && input_manager.fetch_metal_event(MetalEvents::SteelLowBurn)
        {
            self.low_burn(player);
            self.show_particles = true;
        } else if self.was_low_burn {
            self.was_low_burn = false;
            self.object_location = Vector2::ZERO;
            let mut metal_line = player.get_metal_line();
            let mut bound_metal_line = metal_line.bind_mut();
            bound_metal_line.set_should_show(false);
            drop(bound_metal_line);

            metal_line.queue_redraw();
        }

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

impl Steel {
    /// Checks if the passed metal object is the closest metal object to the joystick angle.
    /// If it is then the direction and angle difference are returned.
    /// Otherwise the current object location and angle difference are returned.
    ///
    /// # Arguments
    /// * `metal_object` - A mutable reference to the metal object to check.
    /// * `player_position` - The position of the player.
    /// * `joy_position` - The position of the joystick.
    /// * `current_shortest_angle_diff` - The current shortest angle difference.
    ///
    /// # Returns
    /// * `(Vector2, f32)` - A tuple containing the direction of the metal object and the angle difference.
    fn get_closest_metal_object(
        &mut self,
        player_position: Vector2,
        joy_position: Vector2,
        current_shortest_angle_diff: f32,
        metal_object_position: Vector2,
    ) -> Option<(Vector2, f32)> {
        // Return the current object location and angle difference if joystick is in the deadzone.
        if joy_position.length() < 0.2 {
            return None;
        }

        // Calculate the direction from the player to the metal object.
        let metal_object_dir = (metal_object_position - player_position).normalized();

        // Calculate the absolute angle difference between the joystick direction and this metal object.
        let angle_diff = joy_position.angle_to(metal_object_dir).abs();

        // Only update if this metal object is within the selection cone and closer than our current best.
        if angle_diff < current_shortest_angle_diff {
            return Some((metal_object_dir, angle_diff));
        }

        None
    }
}
