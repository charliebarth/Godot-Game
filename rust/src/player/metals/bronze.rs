//! bronze.rs
//!
//! This file contains the implementation of the Bronze player ability.
//! The Bronze ability allows players to view the particles that other players emit when they burn
//! metals, given that the other player is not burning copper.
//!
//! Author: Michael Imerman, Charles Barth
//! Version: Spring 2025
use crate::player::enums::metal_type::MetalType;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;
use godot::obj::{Gd, GdMut};

/// The Bronze player ability.
/// This ability allows players to view the particles that other players emit when they low burn
/// metals, given that the other player is not burning copper.
pub struct Bronze {
    /// The maximum amount of bronze the player can store.
    capacity: f64,
    /// The current amount of bronze the player has.
    current_reserve: f64,
    /// The rate at which the player burns bronze when using the low burn ability.
    low_burn_rate: f64,
    /// A flag to determine if the player is low burning.
    low_burning: bool,
    /// A flag to determine if the player is burning.
    burning: bool,
    /// A reference to the player.
    player: Gd<Player>,
    /// The type of metal.
    metal_type: MetalType,
    /// The previous amount of bronze the player had.
    previous_reserve: f64,
}

/// Methods for Bronze
impl Bronze {
    /// Creates an instance of the Bronze 
    /// # Arguments
    /// * `capacity` - The maxiumum amount of bronze the player can store 
    /// * `current_reserve` - The current amount of bronze the player has
    /// * `low_burn_rate` - The rate at which the player burns bronze when using
    ///                     the low burn ability
    /// * `player` - A reference to the player
    /// * `metal_type` - The type of metal 
    /// 
    /// # Returns
    /// * An instance of Bronze class 
    pub fn new(
        capacity: f64,
        current_reserve: f64,
        low_burn_rate: f64,
        player: Gd<Player>,
        metal_type: MetalType,
    ) -> Self {
        Self {
            capacity,
            current_reserve,
            previous_reserve: 0.0,
            low_burn_rate,
            low_burning: false,
            burning: false,
            player,
            metal_type,
        }
    }

    /// Function that updates particle visibility for nearby players
    fn update_particle_visibility(&mut self) {
        let mut player = self.player.bind_mut();
        let visibility_mask = 1 << player.get_player_id() * 2;
        let nearby_players = player.get_nearby_players();
        for other_player in nearby_players.iter_mut() {
            let mut other_player = other_player.bind_mut();
            other_player.reveal_particles(visibility_mask);
        }
    }

    /// Function that hides particle visibility for nearby players
    fn hide_particle_visibility(&mut self) {
        let mut player = self.player.bind_mut();
        let visibility_mask = 1 << player.get_player_id() * 2;

        let nearby_players = player.get_nearby_players();
        for other_player in nearby_players.iter_mut() {
            let mut other_player = other_player.bind_mut();
            other_player.hide_particles(visibility_mask);
        }
    }
}


/// Metal methods for Bronze
impl Metal for Bronze {
    /// The burn function for bronze.
    /// It does the same as low_burn because copper has static performance.
    fn burn(&mut self) {
        self.update_particle_visibility();
    }

    /// The low burn function for bronze.
    /// Sets the low_burning flag to true and shows the copper particles.
    fn low_burn(&mut self) {
        if !self.burning {
            self.update_particle_visibility();
        }
    }

    /// This function will update the total metal reserve for bronze.
    ///
    /// # Arguments
    /// * `amount` - The amount to update the reserve by.
    fn update_reserve(&mut self, amount: f64) {
        self.current_reserve += amount;
        self.current_reserve = self.current_reserve.clamp(0.0, self.capacity);
    }

    /// This function will return the type of metal.
    fn metal_type(&self) -> MetalType {
        self.metal_type
    }

    /// This function will return the current reserve of the metal
    fn current_reserve(&self) -> f64 {
        self.current_reserve
    }

    /// This function will return a boolean indicating if the player is burning
    fn burning(&self) -> bool {
        self.burning
    }

    /// This function will return a boolean indicating if the player is low burning
    fn low_burning(&self) -> bool {
        self.low_burning
    }

    /// This function will set the burning flag and update the active metals
    ///
    /// # Arguments
    /// * `burning` - A boolean indicating if the player is burning
    fn set_burning(&mut self, burning: bool) {
        self.burning = burning;

        // check if the player is burning or low burning and add to active metals if they are
        if self.burning || self.low_burning {
            // add the metal to the player's active metals
            let mut player = self.player.bind_mut();
            player.add_active_metal(self.metal_type);
        }

        if !self.burning && !self.low_burning {
            self.hide_particle_visibility();
            // remove the metal from the player's active metals
            let mut player = self.player.bind_mut();
            player.remove_active_metal(self.metal_type);
        }
    }

    /// This function will set the low_burning flag and update the active metals
    ///
    /// # Arguments
    /// * `low_burning` - A boolean indicating if the player is low burning
    fn set_low_burning(&mut self, low_burning: bool) {
        self.low_burning = low_burning;

        // check if the player is burning or low burning and add to active metals if they are
        if self.burning || self.low_burning {
            // add the metal to the player's active metals
            let mut player = self.player.bind_mut();
            player.add_active_metal(self.metal_type);
        }

        if !self.low_burning && !self.burning {
            self.hide_particle_visibility();
            // remove the metal from the player's active metals
            let mut player = self.player.bind_mut();
            player.remove_active_metal(self.metal_type);
        }
    }

    /// This function will get the player
    ///
    /// # Returns
    /// * `GdMut<Player>` - A mutable reference to the player
    fn get_player(&mut self) -> GdMut<'_, Player> {
        self.player.bind_mut()
    }

    /// This function will get the previous reserve of the metal
    ///
    /// # Returns
    /// * `f64` - The previous reserve of the metal
    fn previous_reserve(&self) -> f64 {
        self.previous_reserve
    }

    /// This function will set the previous reserve of the metal
    ///
    /// # Arguments
    /// * `amt` - The amount to set the previous reserve to
    fn set_previous_reserve(&mut self, amt: f64) {
        self.previous_reserve = amt;
    }
}