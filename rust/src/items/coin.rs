/// Represents a coin.
///
/// Author: Trinity Pittman
/// Date: Fall 2024

use godot::prelude::*;
use godot::classes::{IRigidBody2D, RigidBody2D};

use crate::player::enums::coin_events::CoinState;
use crate::player::player::Player;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
#[derive(Debug)]
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
            state: CoinState::Idle,     // Initial state 
            weight: 10,                 
            curr_player: None,          // Initialy no player 
        }
    }

    /// The Godot method called when the coin enters the scene tree for the first time
    /// Any one time logic and initialization should be done here
    /// NOTE: This only is called the very first time the instance enters the scene tree
    /// 
    /// Sets coin freeze mode to true, and allows collsions.
    fn ready(&mut self) {
        godot_print!(
            "{} at position {}",
            self.base().get_name(),
            self.base_mut().get_global_position()
        ); 

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
    /// * `new_player` (Gd<Player>) - The new player to hold the coin 
    pub fn set_curr_player(&mut self, new_player: Gd<Player>) {
        self.curr_player = Some(new_player);
    }

    /// When someone enters this coins hit box we call the method to add a coin to that players  
    /// coin counter.
    ///
    /// # Arguments
    /// * `body` (Gd<Node2D>) - the Node that enters this coin
    #[func]
    fn coin_pickup(&mut self, body: Gd<Node2D>) {
        if self.state == CoinState::Thrown {   // If in state thrown and it hits smth, call drop 
            self.drop(body);

        } else if self.state == CoinState::Idle {   // If in state idle and it hits something
            godot_print!(
                "\n{} pick-up attempt: Body entered -> {}",
                self.base().get_name(),
                body.get_name()
            ); // Debug line

            godot_print!("COIN IN STATE {}", self.state); // Prints coin state 
            let body_name = body.get_name();
            godot_print!("Coin entered by {body_name}"); // Prints who picked up the coin
            
            // See if what entered the coin was a player 
            if let Ok(mut player) = body.try_cast::<Player>() {
                // Update state
                self.set_state(CoinState::PickedUp);
                godot_print!("COIN IN STATE PICKED UP = {}", self.state);

                // Adjust Players coins
                player.bind_mut().adjust_coins(1, self);

                // Keep track of this coins player
                self.set_curr_player(player);
            } else {
                godot_print!("Something other than player entered the coin.");
            }
        }
    }

    /// Handles throwing of the coin, gets direction and applies impulse.
    #[func]
    pub fn throw(&mut self) {
        godot_print!("\nATTEMPTING THROWING {}", self.base().get_name());
        godot_print!("COIN IN STATE {}", self.state);

        // If in PickedUp state
        if self.state == CoinState::PickedUp {
            godot_print!("THROWING {}", self.base().get_name());

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
            godot_print!(
                "REPOSITIONING {} to {} actually {}",
                self.base().get_name(),
                pos,
                real_pos
            );

            // Apply impluse
            godot_print!("Applying impulse {}", force);
            self.base_mut().set_linear_velocity(Vector2::ZERO);
            self.base_mut().set_angular_velocity(0.);
            self.base_mut().apply_impulse(force);

            // Debug physics
            // let velocity = self.base_mut().get_linear_velocity();
            // let sleeping = self.base_mut().is_sleeping();
            // godot_print!("POS: {}", pos);
            // godot_print!("VIS: {}", self.base_mut().is_visible());
            // godot_print!("VEL: {}\nSLEEP: {}", velocity, sleeping);
            // godot_print!("BODIES: {}", self.base_mut().get_colliding_bodies());

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

    /// This method is the way to determine if the object is metal.
    ///
    /// # Returns
    /// * `bool` - True if the object is metal.
    #[func]
    pub fn is_metal(&self) -> bool {
        true
    }
}
