use godot::classes::{IRigidBody2D, InputEvent, RigidBody2D};
/// Represents a coin.
///
/// Author : Trinity Pittman
/// Version : 10/02/2024
use godot::prelude::*;

use crate::player::input_manager::InputManager;
use crate::player::player::Player;

const SPEED: f64 = 25.0;

#[derive(PartialEq)]
pub enum CoinState {
    Idle, 
    PickedUp, 
    Thrown,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum CoinEvents {
    Throw,
    Drop,
}

impl CoinEvents {
    pub fn from_string(button: &str) -> Option<CoinEvents> {
        match button {
            "throw" => Some(CoinEvents::Throw),
            "drop" => Some(CoinEvents::Drop),
            _ => None,
        }
    }
}

/// Represents a coin
#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Coin {
    base: Base<RigidBody2D>,
    state: CoinState,
    weight: i32,
}


#[godot_api]
impl IRigidBody2D for Coin {
    /// Constructor for a Coin
    fn init(base: Base<RigidBody2D>) -> Self {
        Self {  
            base,
            state: CoinState::Idle,
            weight: 10,
        }
    }

    fn ready(&mut self) {
        godot_print!("Coin at position {}", self.base_mut().get_global_position());

        self.base_mut().set_contact_monitor(true);
        self.base_mut().set_max_contacts_reported(1);
    }

    // TODO Unfinished
    fn physics_process(&mut self, delta: f64) {
        if self.state == CoinState::PickedUp {

        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let button_name = InputManager::event_to_input_name(event.clone());

        if let Some(coin_event) = CoinEvents::from_string(&button_name) {
            self.process_coin_events(coin_event, event);
        }

    }
}


#[godot_api]
impl Coin {
    /// When someone enters this coins hit box we call the method to add a coin to that players  
    /// coin counter.
    ///
    /// Args:
    ///      body (Gd<Node2D>): the Node that enters this coin
    #[func]
    fn coin_pickup(&mut self, body: Gd<Node2D>) {
        if self.state == CoinState::Idle {
            self.set_state(CoinState::PickedUp);
            let body_name = body.get_name();
            godot_print!("Coin entered by {body_name}"); // Prints who picked up the coin

            if let Ok(mut player) = body.try_cast::<Player>() {
                player.bind_mut().adjust_coins(1); // Dereference and call the method
                self.base_mut().queue_free(); // Remove the coin from the scene
            } else {
                godot_print!("Something other than player entered the coin.");
            }
            let mut coin = Coin::new_alloc();
        }
    }

    fn set_state(&mut self, state: CoinState) {
        self.state = state;
    }

    #[func]
    pub fn throw(&mut self, force: Vector2, dir: Vector2) {
        // If in PickedUp state
        if self.state == CoinState::PickedUp {
            self.set_state(CoinState::Thrown);
            
            // this is probably dependent on player direction... 
            self.base_mut().apply_impulse(dir * force);
        }
    }

    pub fn drop(&mut self) {
        // drop the coin when it hits something 
        self.set_state(CoinState::Idle);

        // change velocity to zero ? 
    }

    #[func]
    pub fn is_metal(&self) -> bool {
        true // A coin is made of metal
    }

    fn process_coin_events(&mut self, coin_event: CoinEvents, event: Gd<InputEvent>){
        if event.is_action_pressed(StringName::from("throw")) {
            self.throw(Vector2::new(0., 0.), Vector2::new(150., -200.));
        }
    }
}
