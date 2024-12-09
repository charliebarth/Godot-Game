/// Represents a coin counter.
///
/// Author: Trinity Pittman
/// Date: Fall 2024
use godot::{
    classes::{ILabel, InputEvent, Label},
    prelude::*,
};

use crate::{
    items::coin::Coin,
    player::{enums::coin_events::CoinEvents, input_manager::InputManager},
};

// The amount of coins a player starts with
const STARTING_COIN_COUNT: i32 = 0;

#[derive(GodotClass)]
#[class(base=Label)]
/// Struct that represents a Coin Counter
pub struct CoinCounter {
    base: Base<Label>,
    /// The amount of coins
    coins: i32,
    /// Holds Coins
    coin_holder: Vec<Gd<Coin>>,
}

#[godot_api]
/// Godot methods that belong to the CoinCounter
impl ILabel for CoinCounter {
    /// Constructor for the Coin counter
    fn init(base: Base<Label>) -> Self {
        Self {
            base,
            coins: STARTING_COIN_COUNT,
            coin_holder: Vec::new(),
        }
    }

    /// Sets the base value of coins to 10 at the start of the round
    fn ready(&mut self) {
        let coin_cnt = GString::from(format!("{}", self.coins));
        self.base_mut().set_text(coin_cnt.into());

        // for _i in 0..STARTING_COIN_COUNT{
        //     self.coin_holder.push(Coin::new_alloc());
        // }
    }

    /// On an input event, calls the process_coin_events method if the event is a CoinEvent
    /// # Arguments
    /// * `event` (Gd<InputEvent>) - the input event that took place
    fn input(&mut self, event: Gd<InputEvent>) {
        let button_name = InputManager::event_to_input_name(event.clone());

        if let Some(coin_event) = CoinEvents::from_string(&button_name) {
            self.process_coin_events(coin_event, event);
        }
    }
}

#[godot_api]
/// Methods for the CoinCounter
impl CoinCounter {
    /// Increments the number of coins
    /// # Arguments
    /// * `coin` (Coin) - the coin to add to the coin counter
    pub fn add_coin(&mut self, coin: &mut Coin) {
        let new_coins = self.coins + 1; // Find how many coins to change to
        self.base_mut().set_text(new_coins.to_string().into()); // Changes the label text

        // Update coin counter
        self.coins = new_coins;

        // Change the position to outside the map
        let pos = Vector2::new(100000., -100000.);
        let args = &[pos.to_variant()];
        coin.to_gd()
            .call_deferred(StringName::from("set_global_position"), args);
        coin.to_gd()
            .call_deferred(StringName::from("set_freeze_enabled"), &[true.to_variant()]);

        // let real_pos = coin.to_gd().get_global_position();
        // godot_print!("\nREPOSITIONING {} to {} actually {}", coin.to_gd().get_name(), pos, real_pos);

        // Add the coin to the coin holder
        self.coin_holder
            .insert(self.coin_holder.len(), coin.to_gd());
    }

    /// Setter method for the text
    ///
    /// # Arguments
    /// * `text` (String) - The text to set the label to
    fn set_text(&mut self, text: String) {
        let text_g = GString::from(text); // Change the string to a GString for godot
        self.base_mut().set_text(text_g); // set label text
    }

    /// Removes coins from this Coin Counter, returns false if we cannot carry this out
    pub fn remove_coin(&mut self) -> bool {
        let new_coins = self.coins - 1;

        if new_coins < 0 {
            // If we dont have enough coins
            false
        } else {
            self.base_mut().set_text(new_coins.to_string().into()); // Changes the label text
            self.coins = new_coins;
            true
        }
    }

    /// Processes the coin event that happened
    /// # Arguments
    /// * `coin_event` (CoinEvents) - The coin event that took place
    /// * `event` (Gd<InputEvent>) - The input event that took place
    fn process_coin_events(&mut self, coin_event: CoinEvents, event: Gd<InputEvent>) {
        if event.is_action_pressed(StringName::from("throw")) {
            // Check if player has coins to throw
            if (self.remove_coin()) {
                // Get the last coin from the coin holder
                let length = self.coin_holder.len();
                let mut coin = self.coin_holder.remove(length - 1);

                // Throw a coin
                coin.bind_mut().throw();
            }
        }
    }
}
