use godot::classes::{IVBoxContainer, TextureProgressBar, VBoxContainer};
/// UNFINISHED
///
/// Controls the metal bars on screen, stores bars that are not currently on screen and has methods
/// to add and remove bars from the on screen Vbox by name.
///
/// Author : Trinity Pittman
/// Version : 09/22/2024
use godot::prelude::*; // Import Node and VBoxContainer

use std::collections::HashMap;

const MAX_BARS_ON_SCREEN: u8 = 4;

#[derive(GodotClass)]
#[class(base=VBoxContainer)]
pub struct MetalReserveBarManager {
    base: Base<VBoxContainer>,
    /// data structure (like a list) where metals not on screen will be stored
    metal_bars: HashMap<String, Gd<TextureProgressBar>>,
}

/// where the methods that belong to MetalReserveBarManager will be stored
#[godot_api]
impl IVBoxContainer for MetalReserveBarManager {
    // Initialization method for `MetalReserveBarManager`
    fn init(base: Base<VBoxContainer>) -> Self {
        Self {
            base,
            metal_bars: HashMap::new(),
        }
    }

    fn ready(&mut self) {
        let bars: Vec<Gd<TextureProgressBar>> = self
            .base()
            .get_children()
            .iter_shared()
            .map(|child| child.cast::<TextureProgressBar>())
            .collect();

        for bar in bars {
            self.metal_bars.insert(bar.get_name().to_string(), bar);
        }
    }
}

impl MetalReserveBarManager {
    pub fn equip_metal(&mut self, metal: &str) {
        if let Some(metal_node) = self.metal_bars.get_mut(metal) {
            metal_node.set_visible(true);
        }
    }

    pub fn unequip_metal(&mut self, metal: &str) {
        if let Some(metal_node) = self.metal_bars.get_mut(metal) {
            metal_node.set_visible(false);
        }
    }

    pub fn adjust_bar_amount(&mut self, metal: &str, amount: f64) {
        if let Some(metal_node) = self.metal_bars.get_mut(metal) {
            metal_node.set_value(amount);
        }
    }
}
