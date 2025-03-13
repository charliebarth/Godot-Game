use std::collections::VecDeque;

use godot::{
    classes::{IRigidBody2D, RigidBody2D},
    prelude::*,
};

use crate::player::{enums::force::Force, player::Player};

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
        Self {
            base,
            forces: VecDeque::new(),
            delta: 0.0,
            gravity: 0.0,
            air_resistance: 0.0,
            friction: 0.0,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.delta = delta;
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
        let len_forces = self.forces.len();
        let mut expected_force: VecDeque<Force> = VecDeque::new();
        for _ in 0..len_forces {
            let force = self.forces.pop_front().unwrap();
            self.apply_force(force, delta, &mut expected_force);
        }
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
}
