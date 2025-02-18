use godot::builtin::{Color, Vector2};
use godot::classes::Input;
use godot::global::JoyAxis;
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::player::enums::force::Force;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

const NO_BURN_DIRECTION: f32 = 0.0;
const PUSH_BURN_DIRECTION: f32 = -1.0;
const PULL_BURN_DIRECTION: f32 = 1.0;

/// The steel player ability.
/// This ability allows the player to push and pull on metal objects.
/// If the player pushes on a metal object then the player will be pushed in the opposite direction.
/// If the player pulls on a metal object then the player will be pulled in the direction of the metal object.
/// This done by calculating the angle between the player and the metal object and then applying a percentage of the max acceleration
/// based on how far off the angle is from either 0, 90, 180, or 270 degrees.
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Steel {
    /// The base node of the Steel class.
    base: Base<Node2D>,
    /// The maximum amount of steel the player can store.
    capacity: f64,
    /// The current amount of steel the player has.
    current_reserve: f64,
    /// The previous amount of steel the player had.
    previous_reserve: f64,
    /// The rate at which the player burns steel.
    burn_rate: f64,
    /// The rate at which the player burns steel when using the low burn ability.
    low_burn_rate: f64,
    /// A flag to determine if the player was low burning.
    was_low_burn: bool,
    /// The push value for the steel ability.
    /// -1.0 when its a push, 1.0 when its a pull, and 0.0 when its not being used
    burn_direction: f32,
    /// A flag to determine if the player should show the steel particles.
    show_particles: bool,
    /// The location of the metal object that the player is currently pushing or pulling.
    object_location: Vector2,
    /// A flag to determine if the player is low burning.
    low_burn: bool,
    /// A flag to determine if the player is burning.
    burn: bool,
    /// A reference to the player.
    player: Option<Gd<Player>>,
}

#[godot_api]
impl INode2D for Steel {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            capacity: 100.0,
            current_reserve: 0.0,
            previous_reserve: 0.0,
            burn_rate: 1.0,
            low_burn_rate: 0.5,
            was_low_burn: false,
            burn_direction: 0.0,
            show_particles: false,
            object_location: Vector2::ZERO,
            low_burn: false,
            burn: false,
            player: None,
        }
    }

    fn ready(&mut self) {
        self.player = Some(
            self.base()
                .get_parent()
                .expect("parent not found")
                .try_cast::<Player>()
                .expect("player not found"),
        );
    }

    fn physics_process(&mut self, _delta: f64) {
        if self.current_reserve <= 0.0 {
            // If the reserve just hit 0 then turn off the burn and low burn
            if self.current_reserve != self.previous_reserve {
                self.set_burn(false, NO_BURN_DIRECTION);
                self.set_low_burn(false);
            }
            return;
        }

        if self.low_burn {
            self.low_burn();

            // The player can only burn if they are low burning
            if self.burn {
                self.burn();
            }
        }

        // Update the metal reserver bars if the reserve has changed
        if self.current_reserve != self.previous_reserve {
            let mut unbound_player = self.get_player();
            let mut player = unbound_player.bind_mut();

            player.set_metal_reserve_amount(self.as_str().into(), self.current_reserve);
            player.set_metal_reserve_amount("iron".into(), self.current_reserve);
        }

        self.previous_reserve = self.current_reserve;
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
    fn burn(&mut self) {
        let mut unbound_player = self.get_player();
        let mut player = unbound_player.bind_mut();

        if !self.was_low_burn {
            return;
        }

        // Get the normalized direction from the player to the nearest metal object.
        if self.object_location == Vector2::ZERO {
            return;
        }

        player.set_is_steel_burning(true);
        self.current_reserve -= self.burn_rate;
        // TODO: Make constant
        let max_acceleration: f32 = 700.0;

        if !player.base().is_on_floor() {
            player.add_force(Force::NormalForce { magnitude: -1.0 });
        }

        // Use the x and y components directly since 'direction' is normalized.
        let x_velocity = max_acceleration * self.object_location.x * self.burn_direction;
        let y_velocity = max_acceleration * self.object_location.y * self.burn_direction;

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
    fn low_burn(&mut self) {
        let mut unbound_player = self.get_player();
        let mut player = unbound_player.bind_mut();

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

    /// Set the low burn state
    /// If setting to true, the low burn is being started and any one time logic such as revealing the steel particles should be done
    /// If setting to false, the low burn is being stopped and any one time logic such as hiding the steel particles should be done
    ///
    /// # Arguments
    /// * `low_burn` - A boolean to determine if the player should be low burning
    pub fn set_low_burn(&mut self, low_burn: bool) {
        self.low_burn = low_burn;
        if low_burn {
            self.begin_low_burn();
        } else {
            self.cleanup_low_burn();
        }
    }

    /// Set the burn state
    /// If setting to false, the burn is being stopped and thus any effects should be cleaned up and removed
    ///
    /// # Arguments
    /// * `burn` - A boolean to determine if the player should be burning
    /// * `direction` - The direction of the burn (-1.0 for push, 1.0 for pull, 0.0 for no burn)
    pub fn set_burn(&mut self, burn: bool, direction: f32) {
        self.burn = burn;
        self.burn_direction = direction;

        if !burn {
            self.cleanup_burn();
        }
    }

    /// When the player stops low burning, hide the steel particles
    /// and clean remaining metal lines from the screen
    fn cleanup_low_burn(&mut self) {
        let mut unbound_player = self.get_player();
        let mut player = unbound_player.bind_mut();
        player.get_steel_particles().set_visible(false);

        // This will tell the metal line to stop drawing lines and then queue a redraw to clear remaining lines from the screen
        let mut metal_line = player.get_metal_line();
        let mut bound_metal_line = metal_line.bind_mut();
        bound_metal_line.set_should_show(false);
        drop(bound_metal_line);

        metal_line.queue_redraw();
    }

    /// When the player stops burning, set the object location to zero
    /// NOTE: THIS IS PROBABLY WRONG
    fn cleanup_burn(&mut self) {
        self.object_location = Vector2::ZERO;
    }

    /// Get the player
    ///
    /// # Returns
    /// * `Gd<Player>` - A reference to the player
    fn get_player(&self) -> Gd<Player> {
        self.player.as_ref().expect("player not found").clone()
    }

    /// When the player starts low burning, show the steel particles
    fn begin_low_burn(&mut self) {
        let mut unbound_player = self.get_player();
        let mut player = unbound_player.bind_mut();
        player.get_steel_particles().set_visible(true);
    }
}
