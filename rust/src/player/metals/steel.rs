use godot::builtin::{Color, Vector2};
use godot::classes::Input;
use godot::global::JoyAxis;
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::metal_object::MetalObject;
use crate::player::enums::force::Force;
use crate::player::enums::metal_type::MetalType;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

const PUSH_BURN_DIRECTION: f32 = 1.0;

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
    /// The previous amount of steel the player had.
    previous_reserve: f64,
    /// The rate at which the player burns steel.
    burn_rate: f64,
    /// The rate at which the player burns steel when using the low burn ability.
    low_burn_rate: f64,
    /// The push value for the steel ability.
    /// -1.0 when its a push, 1.0 when its a pull, and 0.0 when its not being used
    burn_direction: f32,
    /// The the metal object that the player is currently pushing or pulling.
    object: Option<Gd<MetalObject>>,
    object_location: Vector2,
    /// A flag to determine if the player is low burning.
    low_burn: bool,
    /// A flag to determine if the player is burning.
    burn: bool,
    /// A reference to the player.
    player: Gd<Player>,
    /// The type of metal.
    metal_type: MetalType,
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
        let mut player_clone = self.player.clone();
        let mut player = player_clone.bind_mut();

        if self.object.is_none() {
            return;
        } else if !player.is_metal_object_in_range(self.object.as_ref().unwrap()) {
            self.object = None;
            self.object_location = Vector2::ZERO;
            return;
        }

        self.update_reserve(-self.burn_rate);

        // TODO: Make constant
        let max_acceleration: f32 = 200.0;
        let trigger = if self.metal_type == MetalType::Steel {
            JoyAxis::TRIGGER_RIGHT
        } else {
            JoyAxis::TRIGGER_LEFT
        };

        let strength = Input::singleton().get_joy_axis(player.get_device_id(), trigger);

        // If the player is not on the floor, try to update their up direction based on nearby surfaces.
        if !player.base().is_on_floor() {
            player.add_force(Force::NormalForce { magnitude: -1.0 });
        }

        // Use the x and y components directly since 'direction' is normalized.

        let x_acceleration =
            max_acceleration * self.object_location.x * self.burn_direction * strength;
        let y_acceleration =
            max_acceleration * self.object_location.y * self.burn_direction * strength;

        let metal_object = self.object.as_mut().unwrap();
        let mut bound_metal = metal_object.bind_mut();
        bound_metal.add_force(Force::PlayerSteelPush {
            acceleration: Vector2::new(x_acceleration, y_acceleration),
            player: self.player.clone(),
        });
    }

    /// The low burn function for steel.
    /// This function shows the allomantic lines for the metal objects.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the metal line can be modified.
    fn low_burn(&mut self) {
        self.update_reserve(-self.low_burn_rate);

        if self.burn {
            self.update_selected_object_location();
        } else {
            self.update_line_selection();
        }
    }

    fn update_reserve(&mut self, amount: f64) {
        self.current_reserve += amount;
        self.current_reserve = self.current_reserve.clamp(0.0, self.capacity);
    }

    fn metal_type(&self) -> MetalType {
        self.metal_type
    }

    fn current_reserve(&self) -> f64 {
        self.current_reserve
    }

    fn burning(&self) -> bool {
        self.burn
    }

    fn low_burning(&self) -> bool {
        self.low_burn
    }

    fn set_burning(&mut self, burning: bool) {
        self.burn = burning;

        if !self.burn {
            self.cleanup_burn();
        }
    }

    fn set_low_burning(&mut self, low_burning: bool) {
        self.low_burn = low_burning;

        if self.low_burn {
            // This will tell the metal line to stop drawing lines and then queue a redraw to clear remaining lines from the screen
            let mut player = self.player.bind_mut();
            let mut metal_line = player.get_metal_line(self.metal_type);
            let mut bound_metal_line = metal_line.bind_mut();
            bound_metal_line.set_should_show(true);
            drop(bound_metal_line);
        } else {
            self.cleanup_low_burn();
        }
    }

    fn get_player(&mut self) -> GdMut<'_, Player> {
        self.player.bind_mut()
    }

    fn previous_reserve(&self) -> f64 {
        self.previous_reserve
    }

    fn set_previous_reserve(&mut self, amt: f64) {
        self.previous_reserve = amt;
    }
}

impl Steel {
    pub fn new(
        capacity: f64,
        current_reserve: f64,
        burn_rate: f64,
        low_burn_rate: f64,
        player: Gd<Player>,
        metal_type: MetalType,
    ) -> Self {
        Self {
            capacity,
            current_reserve,
            previous_reserve: 0.0,
            burn_rate,
            low_burn_rate,
            burn_direction: PUSH_BURN_DIRECTION,
            object: None,
            object_location: Vector2::ZERO,
            low_burn: false,
            burn: false,
            player,
            metal_type,
        }
    }

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

    fn update_selected_object_location(&mut self) {
        let mut player = self.player.bind_mut();

        // Get the metal line and show it.
        let mut metal_line = player.get_metal_line(self.metal_type);
        let player_position = metal_line.to_local(metal_line.get_global_position());
        let mut bound_metal_line = metal_line.bind_mut();

        // Get the player position and the index of the closest metal object.
        let mut index_closest_metal_object = usize::MAX;
        let mut points = PackedVector2Array::new();
        let mut colors = PackedColorArray::new();

        for (index, metal_object) in player.get_metal_objects().iter().enumerate() {
            let color = Color::from_rgba(0.0, 0.4, 0.9, 0.1);
            let metal_object_position = bound_metal_line
                .base()
                .to_local(metal_object.get_global_position());

            points.push(player_position);
            points.push(metal_object_position);
            colors.push(color);

            if self.object.is_some() && metal_object == self.object.as_ref().unwrap() {
                if player_position.distance_to(metal_object_position) < 10.0 {
                    self.object = None;
                    self.object_location = Vector2::ZERO;
                }
                {
                    self.object_location = (metal_object_position - player_position).normalized();
                    index_closest_metal_object = index;
                }
            }
        }

        if index_closest_metal_object != usize::MAX {
            colors[index_closest_metal_object] = Color::from_rgba(0.0, 0.6, 2.3, 1.0);
        }

        bound_metal_line.replace_lines(points, colors);
    }

    fn update_line_selection(&mut self) {
        self.object = None;
        self.object_location = Vector2::ZERO;

        let mut closest_object_location: Vector2;

        let mut player_clone = self.player.clone();
        let mut player = player_clone.bind_mut();

        // Get the metal line and show it.
        let mut metal_line = player.get_metal_line(self.metal_type);
        let player_position = metal_line.to_local(metal_line.get_global_position());
        let mut bound_metal_line = metal_line.bind_mut();

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
        let mut points = PackedVector2Array::new();
        let mut colors = PackedColorArray::new();

        for (index, metal_object) in player.get_metal_objects().iter().enumerate() {
            let color = Color::from_rgba(0.0, 0.4, 0.9, 0.1);
            let metal_object_position = bound_metal_line
                .base()
                .to_local(metal_object.get_global_position());

            points.push(player_position);
            points.push(metal_object_position);
            colors.push(color);

            let closest_metal_object = self.get_closest_metal_object(
                player_position,
                joy_position,
                closest_obj_angle_diff,
                metal_object_position,
            );

            if let Some((metal_object_dir, angle_diff)) = closest_metal_object {
                closest_object_location = metal_object_dir;
                closest_obj_angle_diff = angle_diff;
                index_closest_metal_object = index;

                self.object = Some(metal_object.clone());
                self.object_location = closest_object_location;
            }
        }

        if index_closest_metal_object != usize::MAX {
            colors[index_closest_metal_object] = Color::from_rgba(0.0, 0.6, 2.3, 1.0);
        }

        bound_metal_line.replace_lines(points, colors);
    }

    /// When the player stops low burning, hide the steel particles
    /// and clean remaining metal lines from the screen
    fn cleanup_low_burn(&mut self) {
        self.object_location = Vector2::ZERO;
        self.object = None;

        let mut player = self.player.bind_mut();
        player
            .get_metal_particles(self.metal_type)
            .set_visible(false);

        // This will tell the metal line to stop drawing lines and then queue a redraw to clear remaining lines from the screen
        let mut metal_line = player.get_metal_line(self.metal_type);
        let mut bound_metal_line = metal_line.bind_mut();
        bound_metal_line.replace_lines(PackedVector2Array::new(), PackedColorArray::new());
        bound_metal_line.set_should_show(false);
        drop(bound_metal_line);

        metal_line.queue_redraw();
    }

    fn cleanup_burn(&mut self) {
        self.object_location = Vector2::ZERO;
        self.object = None;
    }

    pub fn set_burn_direction(&mut self, direction: f32) {
        self.burn_direction = direction;
    }

    pub fn set_metal_type(&mut self, metal_type: MetalType) {
        self.metal_type = metal_type;
    }
}
