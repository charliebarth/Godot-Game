/// Represents a coin counter. Handles logic for adding and removing coins, and
/// for catching the input event for throwing a coin.
///
/// Author: Trinity Pittman
use godot::{
    classes::{ILabel, InputEvent, Label, Time},
    prelude::*,
};

use crate::{
    items::coin::Coin,
    metal_object::MetalObject,
    player::{
        enums::{
            coin_events::{CoinEvents, CoinState},
            player_events::PlayerEvents,
        },
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
    /// Whether the coin is charging
    charging: bool,
    /// The time the charge started
    charge_start: u64,
    /// The device id of the player
    device_id: i32,
    /// The input manager of the player
    input_manager: Option<Gd<InputManager>>,
    /// The duration of the charge
    charge_duration: u64,
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
            device_id: -1,
            input_manager: None,
            charge_duration: 0,
        }
    }

    /// The Godot method called when the coin counter enters the scene tree for
    /// the first time.
    /// Sets the base value of coins and adds coins to the player.
    fn ready(&mut self) {
        let mut player = self
            .base()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .try_cast::<Player>()
            .unwrap();
        self.device_id = player.bind().get_device_id();
        self.input_manager = Some(player.bind_mut().get_input_manager());
        let coin_cnt = GString::from(format!("{}", self.coins));
        self.base_mut().set_text(&coin_cnt);

        self.add_starting_coins();
    }

    /// This message is called every frame. When the coin charge is full, the
    /// coin will be automatically thrown.
    ///
    /// # Arguments
    /// * `delta` (f64) - The time since the last frame.
    fn process(&mut self, _delta: f64) {
        if let Some(input_manager) = &self.input_manager {
            self.process_coin_events(input_manager.clone());
        }

        if self.charging && Time::singleton().get_ticks_msec() - self.charge_start >= 3000 {
            godot_print!("HIT MAX\n");
            self.charge_duration = 3000;
        }
    }
}

#[godot_api]
/// Methods for the CoinCounter
impl CoinCounter {
    /// Increments the number of coins
    ///
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

    /// Check if the throw event is pressed and if so and not already charging,
    /// start charging. If the throw event is released and we are charging, throw
    /// the coin.
    ///
    /// # Arguments
    /// * `input_manager` (Gd<InputManager>) - The input manager to check for events
    fn process_coin_events(&mut self, mut input_manager: Gd<InputManager>) {
        let bound_input_manager = input_manager.bind_mut();
        if bound_input_manager.check_for_player_event(PlayerEvents::Throw) && !self.charging {
            self.charge_start = Time::singleton().get_ticks_msec();
            self.charging = true;
        } else if !bound_input_manager.check_for_player_event(PlayerEvents::Throw) && self.charging
        {
            if self.charge_duration == 0 {
                self.charge_duration = Time::singleton().get_ticks_msec() - self.charge_start;
            }
            self.throw();
        }
    }

    /// This method checks if the player can throw a coin, if they can it calls
    /// that coins throw method and removes the coin from the coin holder.
    fn throw(&mut self) {
        // Check if player has coins to throw
        if self.remove_coin() {
            // Get the last coin from the coin holder
            let length = self.coin_holder.len();
            let coin_object = self.coin_holder.remove(length - 1);
            let mut coin = coin_object.get_node_as::<Coin>("Coin");

            // Throw a coin
            coin.bind_mut().throw(self.charge_duration as f32);
            self.charging = false;
            self.charge_duration = 0;
        }
    }

    /// Adds the number of coins to start the game depending on the starting
    /// coin count.
    fn add_starting_coins(&mut self) {
        for i in 0..STARTING_COIN_COUNT {
            // Get the coin scene and instantiate it
            let coin_scene = load::<PackedScene>("res://scenes/coin.tscn");
            let mut coin_object = coin_scene.instantiate_as::<MetalObject>().clone();

            // Set the name of the coin
            let coin_id = i as i32 + 1;
            coin_object.set_name(&format!("Coin{}", coin_id));

            // Add the coin to the map (this calls the coin ready method)
            let player = self
                .base()
                .get_parent()
                .unwrap()
                .get_parent()
                .unwrap()
                .get_parent()
                .unwrap()
                .try_cast::<Player>()
                .unwrap();

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
