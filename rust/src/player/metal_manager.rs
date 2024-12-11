use super::{
    metals::{pewter::Pewter, steel::Steel},
    player::Player,
    traits::metal::Metal,
};
use godot::prelude::*;

/// The metal manager is responsible for managing the metals that the player has access to.
/// It creates the metals and assigns them to the player based on the game mode.
/// It also updates the metals every frame.
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MetalManager {
    /// The base node of the MetalManager.
    base: Base<Node2D>,
    /// The metals that the player has access to.
    metals: Vec<Box<dyn Metal>>,
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
            metals: Vec::new(),
        }
    }
}

impl MetalManager {
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
        self.metals
            .push(Box::new(Pewter::new(100.0, 100.0, 0.05, 0.01)));
        self.metals
            .push(Box::new(Steel::new(100.0, 100.0, 0.05, 0.01)));
    }

    /// Updates every metal that the player has access to.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the metals can be updated.
    pub fn update(&mut self, player: &mut Player) {
        for metal in &mut self.metals {
            metal.as_mut().update(player);
        }
    }

    /// Increases the reserve of a specific metal by the given amount.
    ///
    /// # Arguments
    /// * `metal` - The name of the metal to increase the reserve of.
    /// * `amount` - The amount to increase the reserve by.
    pub fn increase_metal_reserve(&mut self, metal: StringName, amount: f64) {
        for m in &mut self.metals {
            if m.as_ref().as_str() == metal.to_string() {
                m.as_mut().increase_reserve(amount);
            }
        }
    }
}
