use std::collections::VecDeque;

use godot::{
    classes::{
        rigid_body_2d::CcdMode, Engine, IRigidBody2D, PhysicsDirectBodyState2D, RigidBody2D,
    },
    prelude::*,
};

use crate::{
    player::{enums::force::Force, player::Player},
    settings::Settings,
};

/// This is a Node for immovable metal objects.
#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct MetalObject {
    /// The base node of the MetalObject.
    base: Base<RigidBody2D>,
    /// A queue of forces to be applied to the metal item
    forces: VecDeque<Force>,
    delta: f64,
    gravity: f64,
    air_resistance: f64,
    friction: f64,
    mass: f32,
}

#[godot_api]
impl IRigidBody2D for MetalObject {
    /// The Godot constructor for the MetalObject class.
    ///
    /// # Arguments
    /// * `base` - The base node of the MetalObject.
    ///
    /// # Returns
    /// * `MetalObject` - A new instance of the MetalObject class.
    fn init(base: Base<RigidBody2D>) -> Self {
        let settings = Engine::singleton()
            .get_singleton("Settings")
            .expect("settings singleton missing")
            .try_cast::<Settings>()
            .expect("settings is not a Settings");

        let settings_bound = settings.bind();
        let gravity: f64 = settings_bound.get_gravity() as f64;
        drop(settings_bound);

        Self {
            base,
            forces: VecDeque::new(),
            delta: 0.0,
            gravity,
            air_resistance: 0.0,
            friction: 0.0,
            mass: 0.1,
        }
    }

    fn ready(&mut self) {
        let mass = self.mass;
        let mut base_mut = self.base_mut();
        base_mut.set_use_custom_integrator(true);
        base_mut.set_contact_monitor(true);
        base_mut.set_max_contacts_reported(16);
        base_mut.set_continuous_collision_detection_mode(CcdMode::CAST_RAY);
        base_mut.set_mass(mass);
    }

    fn physics_process(&mut self, delta: f64) {
        self.delta = delta;
    }

    fn integrate_forces(&mut self, physics_body: Option<Gd<PhysicsDirectBodyState2D>>) {
        if let Some(mut body) = physics_body {
            let mut base_velocity = body.get_linear_velocity();
            base_velocity.y += (self.gravity * self.delta) as f32;
            let result = self.apply_forces(base_velocity);
            base_velocity = result.0;

            let body_position = self.base().get_position();
            for i in 0..body.get_contact_count() {
                let contact_position = body.get_contact_local_position(i);
                let direction = self.determine_collision_direction(body_position, contact_position);
                base_velocity = self.handle_collision(&body, base_velocity, i, direction);
            }

            self.apply_push_back(result.1, base_velocity);
            body.set_linear_velocity(base_velocity);
        }
    }
}

#[godot_api]
impl MetalObject {
    /// This is called when an object impacts the player.
    /// It will calculate if the player should be damaged and if they should be moved.
    /// It returns an impact force which is how much force is returned to the object.
    ///
    /// # Arguments
    /// * `impact_force` - The force of the impact which is roughly calulated using the speed of the object and its weight
    ///
    /// # Returns
    /// * `Force` - A Force::Impact which is how much energy/force is returned to the object,
    /// again roughly calculated using the speed of the player and their weight.
    // pub fn impact(&mut self, impact_force: Force) -> Force {
    //     Force::NormalForce { magnitude: -1.0 }
    // }
    pub fn impact(&mut self, body_mass: f32, body_velocity: Vector2) -> Vector2 {
        let base_velocity = self.base().get_linear_velocity();

        // Compute new velocities separately for X and Y
        let mut new_base_velocity_x = ((self.mass - body_mass) * base_velocity.x
            + 2.0 * body_mass * body_velocity.x)
            / (self.mass + body_mass);

        let mut new_base_velocity_y = ((self.mass - body_mass) * base_velocity.y
            + 2.0 * body_mass * body_velocity.y)
            / (self.mass + body_mass);

        let mut new_body_velocity_x = ((body_mass - self.mass) * body_velocity.x
            + 2.0 * self.mass * base_velocity.x)
            / (self.mass + body_mass);

        let mut new_body_velocity_y = ((body_mass - self.mass) * body_velocity.y
            + 2.0 * self.mass * base_velocity.y)
            / (self.mass + body_mass);

        // Prevent slowing down in the same direction for X
        if new_base_velocity_x.signum() == base_velocity.x.signum()
            && new_base_velocity_x.abs() < base_velocity.x.abs()
        {
            new_base_velocity_x = base_velocity.x;
        }
        if new_body_velocity_x.signum() == body_velocity.x.signum()
            && new_body_velocity_x.abs() < body_velocity.x.abs()
        {
            new_body_velocity_x = body_velocity.x;
        }

        // Prevent slowing down in the same direction for Y
        if new_base_velocity_y.signum() == base_velocity.y.signum()
            && new_base_velocity_y.abs() < base_velocity.y.abs()
        {
            new_base_velocity_y = base_velocity.y;
        }
        if new_body_velocity_y.signum() == body_velocity.y.signum()
            && new_body_velocity_y.abs() < body_velocity.y.abs()
        {
            new_body_velocity_y = body_velocity.y;
        }

        // Apply the corrected velocity to self
        self.add_force(Force::Impact {
            acceleration: Vector2::new(new_base_velocity_x, new_base_velocity_y),
        });

        // Return the new velocity for the body
        Vector2::new(new_body_velocity_x, new_body_velocity_y)
    }

    fn handle_collision(
        &mut self,
        body: &Gd<PhysicsDirectBodyState2D>,
        mut base_velocity: Vector2,
        collision_index: i32,
        direction: &str,
    ) -> Vector2 {
        let collision_body_option = body.get_contact_collider_object(collision_index);

        if let Some(collision_body) = collision_body_option {
            if collision_body.is_class("TileMapLayer") {
                if (direction == "Down" && base_velocity.y.signum() == 1.0)
                    || (direction == "Up" && base_velocity.y.signum() == -1.0)
                {
                    base_velocity.y = 0.0;
                } else if (direction == "Right" && base_velocity.x.signum() == 1.0)
                    || (direction == "Left" && base_velocity.x.signum() == -1.0)
                {
                    base_velocity.x = 0.0;
                }
            } else if collision_body.is_class("Player") {
                let mut player = collision_body.try_cast::<Player>().unwrap();
                base_velocity = player.bind_mut().impact(self.mass, base_velocity);
            } else if collision_body.is_class("MetalObject") {
                let mut metal_object = collision_body.try_cast::<MetalObject>().unwrap();
                base_velocity = metal_object.bind_mut().impact(self.mass, base_velocity);
            }
        }

        base_velocity
    }

    /// This method is the way to determine if the object is metal.
    ///
    /// # Returns
    /// * `bool` - True if the object is metal.
    #[func]
    pub fn is_metal(&self) -> bool {
        true
    }

    pub fn add_force(&mut self, force: Force) {
        self.forces.push_back(force);
    }

    pub fn apply_forces(&mut self, base_velocity: Vector2) -> (Vector2, VecDeque<Force>) {
        let mut base_velocity = base_velocity;
        let mut expected_forces: VecDeque<Force> = VecDeque::new();
        let len_forces = self.forces.len();
        for _ in 0..len_forces {
            let force = self.forces.pop_front().unwrap();
            base_velocity = self.apply_force(force, &mut expected_forces, base_velocity);
        }

        (base_velocity, expected_forces)
    }

    fn apply_push_back(&mut self, expected_forces: VecDeque<Force>, base_velocity: Vector2) {
        for force in expected_forces {
            if let Force::PlayerSteelPush {
                acceleration,
                mut player,
            } = force
            {
                let mut remaining_force = Vector2::ZERO;

                // Compare expected acceleration with actual movement
                // Same direction, but not all force used
                if base_velocity.x.signum() == acceleration.x.signum()
                    && base_velocity.x.abs() < acceleration.x.abs()
                {
                    remaining_force.x = acceleration.x - base_velocity.x;
                }
                // Overcompensation (opposite direction)
                else if base_velocity.x.signum() != acceleration.x.signum() {
                    remaining_force.x = acceleration.x.abs() + base_velocity.x.abs();
                }

                // Same direction, but not all force used
                if base_velocity.y.signum() == acceleration.y.signum()
                    && base_velocity.y.abs() < acceleration.y.abs()
                {
                    remaining_force.y = acceleration.y - base_velocity.y;
                }
                // Overcompensation (opposite direction)
                else if base_velocity.y.signum() != acceleration.y.signum() {
                    remaining_force.y = acceleration.y.abs() + base_velocity.y.abs();
                }

                remaining_force.x = remaining_force.x * acceleration.x.signum();
                remaining_force.y = remaining_force.y * acceleration.y.signum();

                // If any force remains, push it back to the player's force queue
                if remaining_force.length() > 0.0 {
                    player.bind_mut().add_force(Force::SteelPush {
                        x_acceleration: -remaining_force.x * 4.0,
                        y_acceleration: -remaining_force.y * 4.0,
                    });
                }
            }
        }
    }

    pub fn apply_force(
        &mut self,
        force: Force,
        expected_forces: &mut VecDeque<Force>,
        mut base_velocity: Vector2,
    ) -> Vector2 {
        match force {
            Force::PlayerSteelPush {
                acceleration,
                player,
            } => {
                expected_forces.push_back(Force::PlayerSteelPush {
                    acceleration: acceleration.clone(),
                    player,
                });

                base_velocity += acceleration;
            }
            _ => {}
        }

        base_velocity
    }

    fn determine_collision_direction(
        &self,
        body_position: Vector2,
        contact_position: Vector2,
    ) -> &'static str {
        let relative_position = contact_position - body_position;
        let angle = relative_position.y.atan2(relative_position.x).to_degrees();

        match angle {
            a if a >= -45.0 && a < 45.0 => "Right",
            a if a >= 45.0 && a < 135.0 => "Down",
            a if a >= 135.0 || a < -135.0 => "Left",
            _ => "Up",
        }
    }
}
