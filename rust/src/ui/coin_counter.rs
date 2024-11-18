/// Represents a coin counter.
///
/// Author : Trinity Pittman
/// Version : 10/02/2024
use godot::{
    classes::{ILabel, Label},
    prelude::*,
};

// The amount of coins a player starts with
const STARTING_COIN_COUNT: f64 = 10.0;

/// Struct that represents a Coin Counter
#[derive(GodotClass)]
#[class(base=Label)]
pub struct CoinCounter {
    base: Base<Label>,
    /// The amount of coins
    coins: f64,
}

#[godot_api]
impl ILabel for CoinCounter {
    /// Constructor for the Coin counter
    fn init(base: Base<Label>) -> Self {
        Self {
            base,
            coins: STARTING_COIN_COUNT,
        }
    }

    /// Sets the base value of coins to 10 at the start of the round
    fn ready(&mut self) {
        self.base_mut().set_text("10".into());
    }
}

#[godot_api]
impl CoinCounter {
    /// Increments the number of coins
    pub fn add_coin(&mut self) {
        let new_coins = self.coins + 1.; // Find how many coins to change to
        self.base_mut().set_text(new_coins.to_string().into()); // Changes the label text

        // Update coin counter
        self.coins = new_coins;
    }

    /// Setter method for the text
    ///
    /// Args:
    ///     text (String): The text to set the label to
    fn set_text(&mut self, text: String) {
        let text_g = GString::from(text); // Change the string to a GString for godot
        self.base_mut().set_text(text_g); // set label text
    }

    /// Removes coins from this Coin Counter, returns false if we cannot carry this out
    pub fn remove_coin(&mut self) -> bool {
        let new_coins = self.coins - 1.;

        if new_coins < 0.0 {
            // If we dont have enough coins
            false
        } else {
            self.base_mut().set_text(new_coins.to_string().into()); // Changes the label text
            self.coins = new_coins;
            true
        }
    }
}
