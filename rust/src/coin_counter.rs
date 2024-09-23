/// UNFINISHED
/// Represents a coin counter.
/// 
/// Author : Trinity Pittman
/// Version : 09/22/2024

use godot::prelude::*;
use godot::classes::{Label, ILabel};

/// Struct that represents a Coin Counter
#[derive(GodotClass)]
#[class(base=Label)]
pub struct CoinCounter {
    base: Base<Label>,
    /// The amount of Metal reserved in the bar 
    coins: f64,
}


#[godot_api]
impl ILabel for CoinCounter {

    /// Constructor for the Coin counter 
    fn init(base: Base<Label>) -> Self {

        Self {
            base,
            coins: 10.0,
        }
    }

    /// Sets the base value of coins to 10 at the start of the round
    fn ready(&mut self){
        self.base_mut().set_text("10".into());
    }
}

impl CoinCounter {

    /// Increments the number of coins
    pub fn add_coin(&mut self){
        let new_coins = self.coins + 1.;

        self.base_mut().set_text(new_coins.to_string().into());

        // Update coin counter 
        self.coins = new_coins;
    }


    /// Setter method for the text
    pub fn set_text(&mut self, text: String){
        let text_g = GString::from(text);
        self.base_mut().set_text(text_g); 
    }

    /// Removes coins from this Coin Counter 
    fn remove_coins(&mut self, num_coins: f64)-> bool{
        if self.coins - num_coins < 0.0 {
            false
        } else {
            self.coins -= num_coins;
            true
        }
    }


}