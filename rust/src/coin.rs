use godot::classes::{Area2D, IArea2D};
/// UNFINISHED
/// Represents a coin.
///
/// Author : Trinity Pittman
/// Version : 09/22/2024
use godot::prelude::*;

use crate::coin_counter::CoinCounter;

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
}

impl Coin {
    // When someone enters this coins hit box we call the method to add a coin to that players
    // coin counter.
    // fn on_body_entered(&mut self, body: Gd<Node>){

    //     if body.has_method(StringName::from("add_coin")) {
    //         body.call("add_coin".into(), &[]); //This wont work for some reason
    //     }
    //     self.base_mut().queue_free();
    // }
}
