//! Controls the metal bars on screen, stores bars that are not currently on
//! screen and has methods to add and remove bars from the on screen Vbox by
//! name.
//!
//! Author: Trinity Pittman
//! Version: Spring 2025
use std::collections::HashMap;

use godot::classes::{IVBoxContainer, InputMap, VBoxContainer};
use godot::prelude::*;

pub use crate::ui::metal_bar::MetalBar;

// The maximum number of bars to display on a players screen at a time
const MAX_BARS_ON_SCREEN: u8 = 3;

// Change this to account for how many we currently support
const TOTAL_BARS: u8 = 6;

// Represents the order of supported metals (simply reorder these based on implementation)
const PATHS: [&str; 10] = [
    "iron",
    "steel",
    "pewter",
    "tin",
    "bronze",
    "copper",
    "duralumin",
    "nicrosil",
    "chromium",
    "gold",
];

#[derive(GodotClass)]
#[class(base=VBoxContainer)]
/// Represents a manager for the metal bars
pub struct MetalReserveBarManager {
    /// The base node of the MetalReserveBarManager
    base: Base<VBoxContainer>,
    /// Holds the Bars in a hashmap of names and bars
    bars: Option<HashMap<StringName, Gd<MetalBar>>>,
}

#[godot_api]
/// Godot methods that belong to MetalReserveBarManager
impl IVBoxContainer for MetalReserveBarManager {
    /// The Godot contructor for the MetalReserveBarManager class node
    ///
    /// # Arguments
    /// * `base` - The base node type for the MetalReserveBarManager
    ///
    /// # Returns
    /// * `MetalReserveBarManager` - The MetalReserveBarManager node
    fn init(base: Base<VBoxContainer>) -> Self {
        Self { base, bars: None }
    }

    /// The Godot method called when the coin counter enters the scene tree
    /// for the first time.
    /// Creates and sets up the bars inside the Metal Reserve Bar Manager.
    fn ready(&mut self) {
        // Create Metals that are auto added to VBox based on the keybound metals
        for i in 0..TOTAL_BARS + 1 {
            let index = i as usize;

            // Get the name of the metal bar we are creating
            if let Some(&metal_name_temp) = PATHS.get(index) {
                let metal_name = StringName::from(metal_name_temp);

                // Set name of godot object
                self.get_metal_bar(metal_name.clone())
                    .set_name(&metal_name.to_string());

                // Add the bar to VBox
                let metal = self.get_metal_bar(metal_name.clone());
                self.base_mut().add_child(&metal);

                // Set the texture of the bar
                self.get_metal_bar(metal_name.clone())
                    .bind_mut()
                    .set_texture(PATHS[index]);

                // Default hidden
                self.get_metal_bar(metal_name).hide();
            }
        }

        self.setup_keybinds();
    }
}

/// Methods for MetalReserveBarManager
#[godot_api]
impl MetalReserveBarManager {
    /// Sets the metals currently on screen based on the keybindings set
    fn setup_keybinds(&mut self) {
        // Get the input mappings
        let mut input_map: Gd<InputMap> = InputMap::singleton();
        let inputs: Array<StringName> = input_map.get_actions();

        let mut max = 0;
        let length: usize = inputs.len();
        for i in (0..length).rev() {
            let input: StringName = inputs.get(i).unwrap();

            // If the name of the keybind is one of the metals..
            if PATHS.contains(&input.to_string().as_str()) {
                let events: Array<Gd<godot::classes::InputEvent>> =
                    input_map.action_get_events(&input);

                // If something is keybound to the event and not reached max metals, show the bar
                if events.len() > 0 && max != MAX_BARS_ON_SCREEN {
                    self.get_metal_bar(input).show();
                    max = max + 1; // Keeps track of how many are on screen
                }
            }
        }
    }

    /// Given the name of a metal, gets the metal bar associated with it
    ///
    /// # Arguments
    /// * `name` (&str) - the name of the metal bar to get
    ///
    /// # Returns
    /// * (Gd<MetalBar>) - the metal bar or None if none exists
    pub fn get_metal_bar(&mut self, name: StringName) -> Gd<MetalBar> {
        if let Some(bar) = self.get_bars().get(&name) {
            bar.clone()
        } else {
            // Create new bar
            let bar = MetalBar::new_alloc();

            // Add the bar to the hashmap
            self.get_bars().insert(name, bar.clone());
            bar
        }
    }

    /// Gets the HashMap of metal bars, if it doesn't exist, create it
    ///
    /// # Returns
    /// * (HashMap<StringName, Gd<MetalBar>>) - HashMap of MetalBars and their
    ///                                         name (StringName)
    fn get_bars(&mut self) -> &mut HashMap<StringName, Gd<MetalBar>> {
        if self.bars.is_none() {
            self.bars = Some(HashMap::new());
        }
        self.bars.as_mut().unwrap()
    }

    /// Adds metals to all the bars contained within the metal bar manager
    ///
    /// # Arguments
    /// * `metals` (&Vec<StringName>) - the metals to increment
    /// * `amt` (f64) - the ammount to increment by
    pub fn add_metals(&mut self, metals: &Vec<StringName>, amt: f64) {
        for i in 0..self.get_bars().len() {
            // Get the specific bar
            let mut bar = self.get_metal_bar(StringName::from(PATHS[i as usize]));
            if metals.contains(&bar.get_name()) {
                // If its one of the metals
                // add metal reserves
                bar.bind_mut().adjust_reserves(amt);
            }
        }
    }

    // Adds and removes a metal bar from displaying on the screen
    #[func]
    pub fn add_remove(&mut self, unbind: String, bind: String) -> bool {
        if PATHS.contains(&unbind.as_str()) {
            let mut unbind_bar = self.get_metal_bar(unbind.to_lowercase().into());
            let mut bind_bar = self.get_metal_bar(bind.to_lowercase().into());
            if bind_bar.is_visible() {
                return false; // If the bar is already on screen
            } else {
                unbind_bar.hide();
                bind_bar.show();
            }
        }
        true
    }

    /// Sets the amount of a metal in the reserve bar
    ///
    /// # Arguments
    /// * `name` (StringName) - the name of the metal to set the amount of
    /// * `amt` (f64) - the amount to set the metal to
    pub fn set_metal_amount(&mut self, name: &str, amt: f64) {
        let mut bar = self.get_metal_bar(name.into());
        bar.bind_mut().set_value(amt);
    }
}
