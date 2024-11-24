/// Represents a coin counter.
///
/// Author : Trinity Pittman
/// Version : 10/02/2024
use godot::{
    classes::{CharacterBody2D, ILabel, InputEvent, Label},
    prelude::*,
};

use crate::{items::coin::Coin, player::{enums::coin_events::CoinEvents, input_manager::InputManager}};

// The amount of coins a player starts with
const STARTING_COIN_COUNT: i32 = 0;

/// Struct that represents a Coin Counter
#[derive(GodotClass)]
#[class(base=Label)]
pub struct CoinCounter {
    base: Base<Label>,
    /// The amount of coins
    coins: i32,
    coin_holder: Vec<Gd<Coin>>
}

#[godot_api]
impl ILabel for CoinCounter {
    /// Constructor for the Coin counter
    fn init(base: Base<Label>) -> Self {
        Self {
            base,
            coins: STARTING_COIN_COUNT,
            coin_holder: Vec::new()
        }
    }

    /// Sets the base value of coins to 10 at the start of the round
    fn ready(&mut self) {
        let coin_cnt = GString::from(format!("{}", self.coins));
        self.base_mut().set_text(coin_cnt.into());

        // for i in 0..STARTING_COIN_COUNT{
        //     self.coin_holder.push(Coin::new_alloc());
        // }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let button_name = InputManager::event_to_input_name(event.clone());
        
        if let Some(coin_event) = CoinEvents::from_string(&button_name) {
            self.process_coin_events(coin_event, event);
        }
    }
}

#[godot_api]
impl CoinCounter {
    /// Increments the number of coins
    pub fn add_coin(&mut self, coin: &mut Coin) {
        let new_coins = self.coins + 1; // Find how many coins to change to
        self.base_mut().set_text(new_coins.to_string().into()); // Changes the label text

        // Update coin counter
        self.coins = new_coins;

        let pos = Vector2::new(100000., -100000.);
        coin.to_gd().set_global_position(pos);

        let real_pos = coin.to_gd().get_global_position();
        godot_print!("\nREPOSITIONING {} to {} actually {}", coin.to_gd().get_name(), pos, real_pos);

        self.coin_holder.insert(self.coin_holder.len(), coin.to_gd());
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

    fn process_coin_events(&mut self, coin_event: CoinEvents, event: Gd<InputEvent>){
        if event.is_action_pressed(StringName::from("throw")) {
            // Check if player has coins to throw
            if (self.remove_coin()){
                // Get a coin 
                // let mut coin = Coin::new_alloc();

                let player: Gd<CharacterBody2D> = self
                    .base_mut()
                    .get_owner()
                    .unwrap()
                    .get_owner()
                    .unwrap()
                    .cast::<CharacterBody2D>();
                // godot_print!("Parent of coincounter: {}", player.get_name());

                // let pos = player.get_global_position();
                
                // let mut coin = self.coin_holder.last_mut().unwrap();
                let length = self.coin_holder.len();
                let mut coin = self.coin_holder.remove(length-1);
                // coin.set_global_position(pos);

                // Throw a coin
                coin.bind_mut().throw();
            }
            
        }
    }
}
