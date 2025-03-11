use godot::obj::Gd;
use std::ffi::c_void;

use crate::player::enums::metal_type::{BurnType, ButtonState, MetalType};
use crate::player::input_manager::InputManager;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

/// The Copper player ability.
/// This ability allows players to mask their particles from other players.
/// This ability will counter Bronze, which allows players to see other players' particles.
pub struct Copper {
    /// The maximum amount of copper the player can store.
    capacity: f64,
    /// The current amount of copper the player has.
    current_reserve: f64,
    /// The rate at which the player burns copper when using the low burn ability.
    low_burn_rate: f64,
    /// A flag to determine if the player is low burning.
    low_burning: bool,
    /// A flag to determine if the player is burning.
    burning: bool,
    /// A reference to the player.
    player: Gd<Player>,
    /// The type of metal.
    metal_type: MetalType,
    /// The previous amount of copper the player had.
    previous_reserve: f64
}

impl Copper {
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

    /// Function that updates if the player is low burning copper
    pub fn cleanup_lowburn(&mut self) {
        self.low_burning = false;
    }
}

impl Metal for Copper {
    /// The update function for copper.
    /// This function checks to see if the input manager has a copper event.
    /// If the event is found, the low burn function is called.
    /// Will also toggle copper particles on and off.
    fn update(&mut self) {
        let mut input_manager = self.player.bind_mut().get_input_manager();
        self.update_burn(&mut input_manager);
        self.update_low_burn(&mut input_manager);
        if self.current_reserve <= 0.0 {
            // self.cleanup_burn();
            self.cleanup_lowburn();
        } else if self.low_burning {
            self.update_reserve(-self.low_burn_rate);
        }

        if self.current_reserve != self.previous_reserve {
            self.player
                .bind_mut()
                .set_metal_reserve_amount(self.metal_type.as_str(), self.current_reserve);
        }
        self.previous_reserve = self.current_reserve;
    }

    /// The burn function for copper.
    /// It does the same as low_burn because copper has static performance.
    fn burn(&mut self) {
        self.low_burning = true;
    }

    /// The low burn function for copper.
    /// Sets the low_burning flag to true. No particles are shown for copper.
    fn low_burn(&mut self) {
        self.low_burning = true;
    }

    /// This function will update the total metal reserve for copper.
    ///
    /// # Arguments
    /// * `amount` - The amount to update the reserve by.
    fn update_reserve(&mut self, amount: f64) {
        // update the amount of tin the player has
        self.current_reserve += amount;
        // clamp the amount of tin the player has to the capacity
        self.current_reserve = self.current_reserve.clamp(0.0, self.capacity);
    }

    /// This function will return the type of metal.
    ///
    /// # Returns
    /// The type of metal.
    fn metal_type(&self) -> MetalType {
        self.metal_type
    }

    /// This function will update the low burn ability for copper.
    ///
    /// # Arguments
    /// * `input_manager` - The input manager for the player.
    fn update_low_burn(&mut self, input_manager: &mut Gd<InputManager>) {
        let mut input_manager = input_manager.bind_mut();
        let burn_type = BurnType::LowBurn;

        if !self.low_burning
            && input_manager.fetch_metal_event((self.metal_type, burn_type, ButtonState::Pressed))
        {
            self.low_burn();
            self.low_burning = true;
        } else if self.low_burning
            && input_manager.fetch_metal_event((self.metal_type, burn_type, ButtonState::Released))
        {
            self.cleanup_lowburn();
        }
    }

    /// This function will update the burn ability for copper. (Which is the same as low burn)
    ///
    /// # Arguments
    /// * `input_manager` - The input manager for the player.
    fn update_burn(&mut self, input_manager: &mut Gd<InputManager>) {
        self.update_low_burn(input_manager);
    }
}


