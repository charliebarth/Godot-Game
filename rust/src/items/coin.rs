//! Represents a coin.
//!
//! Author: Trinity Pittman
//! Author: Charles Barth

use crate::metal_object::MetalObject;
use crate::player::enums::coin_events::CoinState;
use crate::player::player::Player;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
/// Represents a coin
pub struct Coin {
    // The base node of the Coin
    base: Base<Node2D>,
    /// The state of a Coin
    state: CoinState,
    /// The weight of the coin
    weight: i32,
    /// The current player whose coin counter the coin is in
    curr_player: Option<Gd<Player>>,
    metal_object: Option<Gd<MetalObject>>,
}

#[godot_api]
/// Godot methods for the Coin
impl INode2D for Coin {
    /// The Godot contructor for the Coin class node
    ///
    /// # Arguments
    /// * `base` - The base node type for the coin
    ///
    /// # Returns
    /// * `Coin` - The Coin node
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            state: CoinState::Idle, // Initial state
            weight: 10,
            curr_player: None, // Initialy no player
            metal_object: None,
        }
    }

    /// The Godot method called when the coin enters the scene tree for the
    /// first time. Any one time logic and initialization should be done here
    ///
    /// Sets coin freeze mode to true, and allows collsions.
    fn ready(&mut self) {
        self.metal_object = Some(self.base().get_node_as::<MetalObject>(".."));
        self.base_mut().show(); // Show the coin

        let metal_object = self.metal_object.as_mut().expect("metal object missing");

        metal_object.set_freeze_enabled(true); // Make the coin stay still

        // Emits signals when it collides with another physics body
        metal_object.set_contact_monitor(true);
        // By default this is set to 0, if we want to record contacts it needs to be greater
        metal_object.set_max_contacts_reported(1);
    }

    // fn process(&mut self, _delta: f64) {
    //     let metal_object = self.metal_object.as_ref().unwrap();
    //     let position = self.base().to_local(metal_object.get_position());
    //     self.base_mut().set_position(position);
    // }
}

#[godot_api]
/// Methods that belong to the Coin
impl Coin {
    /// Sets the state of the Coin based on what is passed in
    /// # Arguments
    /// * `new_state` (CoinState) - The new state to set the coin to
    pub fn set_state(&mut self, new_state: CoinState) {
        self.state = new_state;
    }

    /// Sets the current player holding the coin
    /// # Arguments
    /// * `new_player` (`Gd<Player>`) - The new player to hold the coin
    pub fn set_curr_player(&mut self, new_player: Gd<Player>) {
        self.curr_player = Some(new_player);
    }

    /// When someone enters this coins hit box we call the method to add a coin
    /// to that players coin counter.
    ///
    /// # Arguments
    /// * `body` (`Gd<Node2D>`) - the Node that enters this coin
    #[func]
    fn coin_pickup(&mut self, body: Gd<Node2D>) {
        if self.state == CoinState::Thrown {
            // If in state thrown and it hits smth, call drop
            self.drop(body);
        } else if self.state == CoinState::Idle {
            // If in state idle and it hits something

            // See if what entered the coin was a player
            if let Ok(mut player) = body.try_cast::<Player>() {
                // Update state
                self.set_state(CoinState::PickedUp);

                // Adjust Players coins
                player.bind_mut().adjust_coins(1, self.get_metal_obejct());

                // Keep track of this coins player
                self.set_curr_player(player);
            }
        }
    }

    /// Handles throwing of the coin, gets direction and applies impulse.
    ///
    /// # Arguments
    /// * `charge_duration` (f32) - The duration of the charge
    #[func]
    pub fn throw(&mut self, charge_duration: f32) {
        godot_print!("\nATTEMPTING THROWING {}", self.base().get_name());
        godot_print!("COIN IN STATE {}", self.state);

        // If in PickedUp state
        if self.state == CoinState::PickedUp {
            godot_print!("THROWING {}", self.base().get_name());

            let max_charge = 150.;
            let min_charge_rate = 0.1;
            let max_charge_power_time = 3000.;
            let vertical_force = -20.;
            // Calculate charge
            let mut charge = charge_duration as f32 / max_charge_power_time;
            if charge < min_charge_rate {
                charge = min_charge_rate;
            }
            godot_print!("Charge: {}", charge);

            let force;
            let player = self.curr_player.as_mut().unwrap();
            let mut pos = player.get_global_position();
            let x_offset = 20.;
            let y_offset = -15.;

            if player.bind().get_dir() < 0. {
                // Throw left
                force = Vector2::new(-1. * (max_charge * charge), vertical_force);
                pos = pos + Vector2::new(-x_offset, y_offset); // Adjust pos to be higher
            } else {
                // Throw right
                force = Vector2::new(max_charge * charge, vertical_force);
                pos = pos + Vector2::new(x_offset, y_offset); // Adjust pos for throwing right
            }

            let metal_object = self.metal_object.as_mut().expect("metal object missing");
            metal_object.set_freeze_enabled(false);
            metal_object.set_global_position(pos); // Set position to the player
            metal_object.set_visible(true); // Ensure visible
            metal_object.set_sleeping(false); // Ensure awake

            // Apply impluse
            metal_object.set_linear_velocity(Vector2::ZERO);
            metal_object.set_angular_velocity(0.);
            metal_object.apply_impulse(force);

            // Update state
            self.set_state(CoinState::Thrown);
        }
    }

    /// Handles dropping the coin, called when the coin enters something while
    /// in throw state. If the coin enters a player other than the player who
    /// threw the coin, damages the player.
    pub fn drop(&mut self, body: Gd<Node2D>) {
        // If the player the coin entered is not the current player
        if let Ok(mut player) = body.try_cast::<Player>() {
            if player.get_name() != self.curr_player.as_ref().unwrap().get_name() {
                // Hurt the player
                player.bind_mut().adjust_health(-10.);
                // change the state to idle
                self.set_state(CoinState::Idle);
            }
        // If the player the coin entered is the current player
        } else {
            // Change the state to idle
            self.set_state(CoinState::Idle);
        }
    }

    #[func]
    /// Returns true if the coin is a metal object
    ///
    /// # Returns
    /// * `bool` - True if the coin is a metal object
    fn is_metal(&self) -> bool {
        true
    }

    /// Returns the metal object associated with the coin
    ///
    /// # Returns
    /// * `Gd<MetalObject>` - The metal object associated with the coin
    pub fn get_metal_obejct(&self) -> Gd<MetalObject> {
        self.metal_object.clone().unwrap()
    }
}
