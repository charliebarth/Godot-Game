use godot::obj::bounds::MemManual;
use godot::obj::NewAlloc;
/// Represents a coin.
///
/// Author : Trinity Pittman
/// Version : 10/02/2024
use godot::prelude::*;

use crate::player::player::Player;

const SPEED: f64 = 25.0;

/// Represents a coin
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Coin {
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Coin {
    /// Constructor for a Coin
    fn init(base: Base<Area2D>) -> Self {
        Self { base }
    }

    // TODO Unfinished
    fn physics_process(&mut self, delta: f64) {
        let transform: Transform2D = self.base_mut().get_global_transform();
        let position: Vector2 = self.base_mut().get_position();

        // let new_pos: Vector2 = idk maybe tranform.x * speed
        // self.base_mut().set_position(position + new_pos);
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

    #[func]
    pub fn is_metal(&self) -> bool {
        true // A coin is made of metal
    }
}
