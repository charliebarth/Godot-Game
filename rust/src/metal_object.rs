use std::collections::VecDeque;

use godot::{
    classes::{Engine, IRigidBody2D, PhysicsDirectBodyState2D, RigidBody2D},
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
        }
    }

    fn ready(&mut self) {
        let mut base_mut = self.base_mut();
        base_mut.set_use_custom_integrator(true);
        base_mut.set_contact_monitor(true);
        base_mut.set_max_contacts_reported(8);
    }

    fn physics_process(&mut self, delta: f64) {
        self.delta = delta;
    }

    fn integrate_forces(&mut self, physics_body: Option<Gd<PhysicsDirectBodyState2D>>) {
        if let Some(mut body) = physics_body {
            let mut base_velocity = body.get_linear_velocity();
            base_velocity.y += (self.gravity * self.delta) as f32;
            body.set_linear_velocity(base_velocity);

            let body_position = self.base().get_position();
            for i in 0..body.get_contact_count() {
                let contact_position = body.get_contact_local_position(i);
                let direction = self.determine_collision_direction(body_position, contact_position);

                if base_velocity.y.signum() == 1.0 && direction == "Down" {
                    let collision_body_option = body.get_contact_collider_object(i);

                    if let Some(collision_body) = collision_body_option {
                        if collision_body.is_class("TileMapLayer") {
                            base_velocity.y = 0.0;
                        } else if collision_body.is_class("Player") {
                            let player = collision_body.try_cast::<Player>().unwrap();
                            // call player.bind_mut().impact( STUFF HERE )
                        } else if collision_body.is_class("MetalObject") {
                            let metal_object = collision_body.try_cast::<MetalObject>().unwrap();
                            // call metal_object.bind_mut().impact( STUFF HERE )
                        }
                    }
                } else if base_velocity.y.signum() == -1.0 && direction == "Up" {
                }

                if base_velocity.x.signum() == 1.0 && direction == "Right" {
                } else if base_velocity.x.signum() == -1.0 && direction == "Left" {
                }
            }
        }
    }
}

#[godot_api]
impl MetalObject {
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

    pub fn apply_forces(&mut self, delta: f64) {
        // let len_forces = self.forces.len();
        // let mut expected_force: VecDeque<Force> = VecDeque::new();

        // for _ in 0..len_forces {
        //     let force = self.forces.pop_front().unwrap();
        //     self.apply_force(force, delta, &mut expected_force);
        // }

        let mut base_velocity = self.base().get_linear_velocity();

        base_velocity.y += (self.gravity * delta) as f32;

        // let colliding_bodies = self.base().get_colliding_bodies();

        // for body in colliding_bodies.iter_shared() {
        //     if body.is_class("TileMapLayer") {
        //     } else if body.is_class("Player") {
        //     } else if body.is_class("MetalObject") {
        //     }
        // }
    }

    pub fn apply_force(&mut self, force: Force, delta: f64, expected_forces: &mut VecDeque<Force>) {
        // This is a queue of pushes and pulls that we will iterate through after all forces have been resolved.
        // We check each acceleration against the actual acceleration of the object to see how much
        // resistance was experienced or how much of the force was used.
        // Unused force will be returned to the player associated with the push/pull

        let mut base_velocity = self.base().get_linear_velocity();

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

        // Should call integrate_forces instead of set_linear_velocity
        //self.base_mut().set_linear_velocity(base_velocity);
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
