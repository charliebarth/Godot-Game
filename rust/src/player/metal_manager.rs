use std::collections::HashMap;

use super::{
    enums::metal_type::MetalType,
    metals::{iron::Iron, pewter::Pewter, steel::Steel},
    player::Player,
    traits::metal::Metal,
};
use godot::prelude::*;

/// The metal manager is responsible for managing the metals that the player has access to.
/// It creates the metals and assigns them to the player based on the game mode.
/// It also updates the metals every frame.
#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct MetalManager {
    /// The base node of the MetalManager.
    base: Base<Node2D>,
    /// The metals that the player has access to.
    metals: HashMap<MetalType, Box<dyn Metal>>,
    /// The player that the metal manager is attached to.
    player: Option<Gd<Player>>,
}

#[godot_api]
impl INode2D for MetalManager {
    /// The Godot constructor for the MetalManager class.
    ///
    /// # Arguments
    /// * `base` - The base node of the MetalManager.
    ///
    /// # Returns
    /// * `MetalManager` - A new instance of the MetalManager class.
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            metals: HashMap::new(),
            player: None,
        }
    }

    fn ready(&mut self) {
        let player_node = self.base().get_parent().expect("parent not found");
        let player = player_node.try_cast::<Player>().expect("player not found");

        self.player = Some(player);
    }

    fn physics_process(&mut self, _delta: f64) {
        self.update_metals();
    }
}

impl MetalManager {
    pub fn set_player(&mut self, player: Gd<Player>) {
        self.player = Some(player);
    }

    /// Assigns the starting metals to the player based on the game mode.
    /// The match statement will be expanded in the future to include more game modes.
    ///
    /// # Arguments
    /// * `game_mode` - The game mode that the player is playing.
    pub fn assign_starting_metals(&mut self, game_mode: &str) {
        match game_mode {
            "last_player_standing" => self.last_player_standing(),
            _ => {}
        }
    }

    /// Assigns the metals for the last player standing game mode.
    fn last_player_standing(&mut self) {
        let player = self.player.as_ref().unwrap();

        self.metals.insert(
            MetalType::Pewter,
            Box::new(Pewter::new(
                100.0,
                100.0,
                0.05,
                0.01,
                player.clone(),
                MetalType::Pewter,
            )),
        );
        self.metals.insert(
            MetalType::Steel,
            Box::new(Steel::new(
                100.0,
                100.0,
                0.05,
                0.01,
                player.clone(),
                MetalType::Steel,
            )),
        );
        self.metals.insert(
            MetalType::Iron,
            Box::new(Iron::new(
                100.0,
                100.0,
                0.05,
                0.01,
                player.clone(),
                MetalType::Iron,
            )),
        );
    }

    /// Updates every metal that the player has access to.
    pub fn update_metals(&mut self) {
        for metal in &mut self.metals {
            let metal = metal.1;
            metal.update_low_burn();
            metal.update_burn();

            if metal.low_burning() {
                metal.low_burn();
            }

            if metal.burning() {
                metal.burn();
            }

            if metal.current_reserve() != metal.previous_reserve() {
                let metal_type = metal.metal_type();
                let metal_type = metal_type.as_str();
                let current_reserve = metal.current_reserve();

                metal
                    .get_player()
                    .set_metal_reserve_amount(metal_type, current_reserve);
            }

            metal.set_previous_reserve(metal.current_reserve());
        }
    }

    /// Increases the reserve of a specific metal by the given amount.
    ///
    /// # Arguments
    /// * `metal` - The name of the metal to increase the reserve of.
    /// * `amount` - The amount to increase the reserve by.
    pub fn increase_metal_reserve(&mut self, metal: &str, amount: f64) {
        let metal_type = MetalType::from_string(metal);

        if let Some(metal_type) = metal_type {
            self.metals
                .get_mut(&metal_type)
                .unwrap()
                .update_reserve(amount);
        }
    }
}
