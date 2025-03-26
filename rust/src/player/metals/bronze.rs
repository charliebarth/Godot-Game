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

impl Bronze {
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
        let visibility_mask = 1 << player.get_player_id();
        let nearby_players = player.get_nearby_players();
        for other_player in nearby_players.iter_mut() {
            let mut other_player = other_player.bind_mut();
            other_player.reveal_particles(visibility_mask);
        }
    }

    fn hide_particle_visibility(&mut self) {
        let mut player = self.player.bind_mut();

        let nearby_players = player.get_nearby_players();
        for other_player in nearby_players.iter_mut() {
            let mut other_player = other_player.bind_mut();
            other_player.hide_particles();
        }
    }
}

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

    fn current_reserve(&self) -> f64 {
        self.current_reserve
    }

    fn burning(&self) -> bool {
        self.burning
    }

    fn low_burning(&self) -> bool {
        self.low_burning
    }

    fn set_burning(&mut self, burning: bool) {
        self.burning = burning;

        if !self.burning && !self.low_burning {
            self.hide_particle_visibility();
        }
    }
    fn set_low_burning(&mut self, low_burning: bool) {
        self.low_burning = low_burning;

        if !self.low_burning && !self.burning {
            self.hide_particle_visibility();
        }
    }

    fn get_player(&mut self) -> GdMut<'_, Player> {
        self.player.bind_mut()
    }

    fn previous_reserve(&self) -> f64 {
        self.previous_reserve
    }

    fn set_previous_reserve(&mut self, amt: f64) {
        self.previous_reserve = amt;
    }
}
