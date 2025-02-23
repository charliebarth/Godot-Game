use godot::classes::{IRigidBody2D, RigidBody2D};
/// Represents a coin.
///
/// Author: Trinity Pittman
/// Date: Fall 2024
use godot::prelude::*;

use crate::player::enums::coin_events::CoinState;
use crate::player::player::Player;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
/// Represents a coin
pub struct Coin {
    // The base node of the Coin
    base: Base<RigidBody2D>,
    /// The state of a Coin
    state: CoinState,
    /// The weight of the coin
    weight: i32,
    /// The current player whose coin counter the coin is in
    curr_player: Option<Gd<Player>>,
}

#[godot_api]
/// Godot methods for the Coin
impl IRigidBody2D for Coin {
    /// The Godot contructor for the Coin class node
    ///
    /// # Arguments
    /// * `base` - The base node type for the coin
    ///
    /// # Returns
    /// * `Coin` - The Coin node
    fn init(base: Base<RigidBody2D>) -> Self {
        Self {
            base,
            state: CoinState::Idle, // Initial state
            weight: 10,
            curr_player: None, // Initialy no player
        }
    }

    /// The Godot method called when the coin enters the scene tree for the first time
    /// Any one time logic and initialization should be done here
    ///
    /// Sets coin freeze mode to true, and allows collsions.
    fn ready(&mut self) {
        self.base_mut().show(); // Show the coin

        self.base_mut().set_freeze_enabled(true); // Make the coin stay still

        // Emits signals when it collides with another physics body
        self.base_mut().set_contact_monitor(true);
        // By default this is set to 0, if we want to record contacts it needs to be greater
        self.base_mut().set_max_contacts_reported(1);
    }
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

    /// When someone enters this coins hit box we call the method to add a coin to that players  
    /// coin counter.
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

            let body_name = body.get_name();

            // See if what entered the coin was a player
            if let Ok(mut player) = body.try_cast::<Player>() {
                // Update state
                self.set_state(CoinState::PickedUp);

                // Adjust Players coins
                player.bind_mut().adjust_coins(1, self);

                // Keep track of this coins player
                self.set_curr_player(player);
            }
        }
    }

    /// Handles throwing of the coin, gets direction and applies impulse.
    #[func]
    pub fn throw(&mut self) {
        // If in PickedUp state
        if self.state == CoinState::PickedUp {
            let force;
            let player = self.curr_player.as_mut().unwrap();
            let mut pos = player.get_global_position();

            if player.bind().get_dir() < 0. {
                // Throw left
                force = Vector2::new(-500., -400.);
                pos = pos + Vector2::new(0., -5.); // Adjust pos to be higher
            } else {
                // Throw right
                force = Vector2::new(500., -400.);
                pos = pos + Vector2::new(20., -15.); // Adjust pos for throwing right
            }

            self.base_mut().set_freeze_enabled(false);
            self.base_mut().set_global_position(pos); // Set position to the player
            self.base_mut().set_visible(true); // Ensure visible
            self.base_mut().set_sleeping(false); // Ensure awake

            // Debugging
            let real_pos = self.base_mut().get_global_position();

            // Apply impluse

            self.base_mut().set_linear_velocity(Vector2::ZERO);
            self.base_mut().set_angular_velocity(0.);
            self.base_mut().apply_impulse(force);

            // Update state
            self.set_state(CoinState::Thrown);
        }
    }

    /// Handles dropping the coin, called when the coin enters something while in throw state. If
    /// the coin enters a player other than the player who threw the coin, damages the player.
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
    fn is_metal(&self) -> bool {
        true
    }
}
