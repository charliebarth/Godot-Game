/// Represents a coin counter.
///
/// Author: Trinity Pittman
/// Date: Fall 2024
use godot::{
    classes::{ILabel, InputEvent, Label, Time},
    prelude::*,
};

use crate::{
    items::coin::Coin,
    metal_object::MetalObject,
    player::{
        enums::coin_events::{CoinEvents, CoinState},
        input_manager::InputManager,
        player::Player,
    },
};

// The amount of coins a player starts with
const STARTING_COIN_COUNT: i32 = 10;

#[derive(GodotClass)]
#[class(base=Label)]
/// Struct that represents a Coin Counter
pub struct CoinCounter {
    // The base node of the CoinCounter
    base: Base<Label>,
    /// The amount of coins
    coins: i32,
    /// Holds Coins
    coin_holder: Vec<Gd<MetalObject>>,
    charging: bool,
    charge_start: u64,
}

#[godot_api]
/// Godot methods that belong to the CoinCounter
impl ILabel for CoinCounter {
    /// The Godot contructor for the CoinCounter class node
    ///
    /// # Arguments
    /// * `base` - The base node type for the CoinCounter
    ///
    /// # Returns
    /// * `CoinCounter` - The CoinCounter node
    fn init(base: Base<Label>) -> Self {
        Self {
            base,
            coins: 0,
            coin_holder: Vec::new(), // Create a new vector to hold coins
            charging: false,
            charge_start: 0,
        }
    }

    /// The Godot method called when the coin counter enters the scene tree for the first time
    /// Sets the base value of coins and adds coins to the player.
    fn ready(&mut self) {
        let coin_cnt = GString::from(format!("{}", self.coins));
        self.base_mut().set_text(&coin_cnt);

        self.add_starting_coins();
    }

    fn process(&mut self, _delta: f64) {
        if self.charging && Time::singleton().get_ticks_msec() - self.charge_start >= 3000 {
            godot_print!("HIT MAX\n");
            self.throw();
        }
    }

    /// On an input event, calls the process_coin_events method if the event is a CoinEvent
    /// # Arguments
    /// * `event` (`Gd<InputEvent>`) - the input event that took place
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
    pub fn add_coin(&mut self, mut coin: Gd<MetalObject>) {
        let new_coins = self.coins + 1; // Find how many coins to change to
        self.base_mut().set_text(&new_coins.to_string()); // Changes the label text

        // Update coin counter
        self.coins = new_coins;

        coin.set_visible(true);

        // Change the position to outside the map
        let pos = Vector2::new(100000., -100000.);
        let args = &[pos.to_variant()];
        coin.call_deferred("set_global_position", args);

        // Enable freeze mode
        coin.call_deferred("set_freeze_enabled", &[true.to_variant()]);

        // Add the coin to the coin holder
        self.coin_holder.insert(self.coin_holder.len(), coin);
    }

    /// Setter method for the text
    ///
    /// # Arguments
    /// * `text` (String) - The text to set the label to
    fn set_text(&mut self, text: String) {
        let text_g = GString::from(text); // Change the string to a GString for godot
        self.base_mut().set_text(&text_g); // set label text
    }

    /// Removes coins from this Coin Counter, returns false if we cannot carry this out
    /// # Returns
    /// * boolean of whether the coin can be removed
    pub fn remove_coin(&mut self) -> bool {
        let new_coins = self.coins - 1;

        if new_coins < 0 {
            // If we dont have enough coins
            false
        } else {
            self.base_mut().set_text(&new_coins.to_string()); // Changes the label text
            self.coins = new_coins;
            true
        }
    }

    /// Processes the coin event that happened
    /// # Arguments
    /// * `coin_event` (CoinEvents) - The coin event that took place
    /// * `event` (Gd<InputEvent>) - The input event that took place
    fn process_coin_events(&mut self, _coin_event: CoinEvents, event: Gd<InputEvent>) {
        if event.is_action_pressed("throw") {
            if !self.charging {
                self.charge_start = Time::singleton().get_ticks_msec();
                self.charging = true;
            }
        }
        if event.is_action_released("throw") && self.charging {
            self.throw();
        }
    }

    fn throw(&mut self) {
        // Check if player has coins to throw
        if self.remove_coin() {
            // Get the last coin from the coin holder
            let length = self.coin_holder.len();
            let coin_object = self.coin_holder.remove(length - 1);
            let mut coin = coin_object.get_node_as::<Coin>("Coin");

            // Throw a coin
            coin.bind_mut()
                .throw(self.charge_start, Time::singleton().get_ticks_msec());
            self.charging = false;
        }
    }

    /// Adds the number of coins to start the game depending on the starting coin count.
    fn add_starting_coins(&mut self) {
        for i in 0..STARTING_COIN_COUNT {
            // Get the coin scene and instantiate it
            let coin_scene = load::<PackedScene>("res://scenes/coin.tscn");
            let mut coin_object = coin_scene.instantiate_as::<MetalObject>().clone();

            // Set the name of the coin
            let coin_id = i as i32 + 1;
            coin_object.set_name(&format!("Coin{}", coin_id));

            // Add the coin to the map (this calls the coin ready method)
            let player = self.base().get_node_as::<Player>("../../");
            let mut map = player.get_parent().expect("Failed to add coin");
            map.add_child(&coin_object);

            let mut coin = coin_object.get_node_as::<Coin>("Coin");

            // Get the player and set the coins current player
            let mut bound_coin = coin.bind_mut();
            bound_coin.set_curr_player(player.to_godot());

            // Set initial state
            bound_coin.set_state(CoinState::PickedUp);

            self.add_coin(coin_object);
        }
    }
}
