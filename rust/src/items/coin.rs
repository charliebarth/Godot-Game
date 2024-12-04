use std::borrow::Borrow;

use godot::classes::rigid_body_2d::FreezeMode;
use godot::classes::{CharacterBody2D, IRigidBody2D, InputEvent, RigidBody2D};
/// Represents a coin.
///
/// Author : Trinity Pittman
/// Version : 10/02/2024
use godot::prelude::*;

use crate::player::input_manager::InputManager;
use crate::player::player::Player;
use crate::player::enums::coin_events::{CoinState, CoinEvents};

const SPEED: f64 = 25.0;

/// Represents a coin
#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Coin {
    base: Base<RigidBody2D>,
    state: CoinState,
    weight: i32,
    curr_player: Option<Gd<Player>>
}


#[godot_api]
impl IRigidBody2D for Coin {
    /// Constructor for a Coin
    fn init(base: Base<RigidBody2D>) -> Self {
        Self {  
            base,
            state: CoinState::Idle,
            weight: 10,
            curr_player: None,
        }
    }

    fn ready(&mut self) {
        godot_print!("{} at position {}", self.base().get_name(), self.base_mut().get_global_position());
        self.base_mut().show();

        
        self.base_mut().set_freeze_enabled(true);
        self.set_state(CoinState::Idle);     
           
        
        self.base_mut().set_contact_monitor(true);
        self.base_mut().set_max_contacts_reported(1);
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
        

        if self.state == CoinState::Thrown {
            if let Ok(mut player) = body.try_cast::<Player>() {
                if player.get_name() != self.curr_player.as_ref().unwrap().get_name() {
                    player.bind_mut().adjust_health(-10.);
                    self.drop()
                }
            } else {
                self.drop()
            }
        } else if self.state == CoinState::Idle {
            godot_print!("\n{} pick-up attempt: Body entered -> {}", self.base().get_name(), body.get_name());  // Debug line
            godot_print!("COIN IN STATE {}", self.state);
            let body_name = body.get_name();
            godot_print!("Coin entered by {body_name}"); // Prints who picked up the coin

            if let Ok(mut player) = body.try_cast::<Player>() {
                self.set_state(CoinState::PickedUp);
                godot_print!("COIN IN STATE PICKED UP = {}", self.state);


                // let mut args = &[1.to_variant(), self.base_mut().to_variant()];
                
                // player.call_deferred(StringName::from("adjust_coins"), args);
                player.bind_mut().adjust_coins(1, self); // Dereference and call the method
                // let pos = Vector2::new(100000., -100000.);
                // self.base_mut().set_global_position(pos);
                

                // let real_pos = self.base_mut().get_global_position();
                // godot_print!("REPOSITIONING pickup to {} actually {}", pos, real_pos);
                self.curr_player = Some(player);
                // self.base_mut().queue_free(); // Remove the coin from the scene
            } else {
                godot_print!("Something other than player entered the coin.");
            }
        }
    
    }

    fn defer_adjust_coins(&mut self, mut player: Player, amount: i8){
        player.adjust_coins(amount, self)
    }


    fn set_state(&mut self, new_state: CoinState) {
        self.state = new_state;
    }


    #[func]
    pub fn throw(&mut self) {
        
        godot_print!("\nATTEMPTING THROWING {}", self.base().get_name());
        godot_print!("COIN IN STATE {}", self.state);

        // If in PickedUp state
        if self.state == CoinState::PickedUp {
            godot_print!("THROWING");
            
            let force;
            let player = self.curr_player.as_mut().unwrap();
            let mut pos = player.get_global_position();
            // let position = player.to_local(pos);

            if (player.bind().get_dir() < 0.) {
                force = Vector2::new(-500., -400.);
            } else {
                force = Vector2::new(500., -400.);
                pos = pos + Vector2::new(20., 0.);
            }

            self.base_mut().set_freeze_enabled(false);
            self.base_mut().set_global_position(pos);
            let real_pos = self.base_mut().get_global_position();

            godot_print!("REPOSITIONING {} to {} actually {}", self.base().get_name(), pos, real_pos);
            godot_print!("Applying impulse {}", force);
            // self.base_mut().set_center_of_mass(pos);
            self.base_mut().apply_impulse(force);

            self.set_state(CoinState::Thrown);
        }
    }

    pub fn drop(&mut self) {
        // drop the coin when it hits something 
        
        self.set_state(CoinState::Idle);
        // self.base_mut().set_axis_velocity(Vector2::ZERO);
        self.base_mut().set_freeze_enabled(true);


        // change velocity to zero ? 
    }

    // #[func]
    // pub fn is_metal(&self) -> bool {
    //     true // A coin is made of metal
    // }
}
