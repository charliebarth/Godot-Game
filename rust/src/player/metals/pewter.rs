use godot::obj::Gd;

use crate::player::enums::metal_type::{BurnType, ButtonState, MetalType};
use crate::player::input_manager::InputManager;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

/// The pewter player ability.
/// CUrrently this gives the player a speed boost and a jump boost.
/// In the future I would like to reduce these boost and add additional mechanics.
/// Such as the ability to wall jump or chain a landing into a high jump.
pub struct Pewter {
    /// The maximum amount of pewter the player can store.
    capacity: f64,
    /// The current amount of pewter the player has.
    current_reserve: f64,
    /// The rate at which the player burns pewter.
    burn_rate: f64,
    /// The rate at which the player burns pewter when using the low burn ability.
    low_burn_rate: f64,
    /// A reference to the player.
    player: Gd<Player>,
    /// The type of metal.
    metal_type: MetalType,
    /// A flag to determine if the player is currently burning pewter.
    burning: bool,
    /// A flag to determine if the player is currently low burning pewter.
    low_burning: bool,
}

impl Pewter {
    /// The constructor for the pewter struct.
    ///
    /// # Arguments
    /// * `capacity` - The maximum amount of pewter the player can store.
    /// * `current_reserve` - The current amount of pewter the player has.
    /// * `burn_rate` - The rate at which the player burns pewter.
    /// * `low_burn_rate` - The rate at which the player burns pewter when using the low burn ability.
    ///
    /// # Returns
    /// * `Pewter` - A new instance of the pewter struct.
    pub fn new(
        capacity: f64,
        current_reserve: f64,
        burn_rate: f64,
        low_burn_rate: f64,
        player: Gd<Player>,
        metal_type: MetalType,
    ) -> Self {
        // player
        //     .bind_mut()
        //     .set_metal_reserve_amount(MetalType::Steel.as_str().into(), current_reserve);

        Self {
            capacity,
            current_reserve,
            burn_rate,
            low_burn_rate,
            player,
            metal_type,
            burning: false,
            low_burning: false,
        }
    }

    fn cleanup_burn(&mut self) {
        self.burning = false;
        // Remove movement buff from player
    }

    fn cleanup_lowburn(&mut self) {
        self.low_burning = false;
        self.player
            .bind_mut()
            .get_pewter_particles()
            .set_visible(false);
        // Remove movement buff from player
    }
}

impl Metal for Pewter {
    /// The burn function for pewter.
    /// This function gives the player a large speed boost and a large jump boost.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the run speed and jump force can be modified.
    fn burn(&mut self) {
        self.update_reserve(-self.burn_rate);
        let mut player = self.player.bind_mut();

        let run_speed = player.get_run_speed();
        let jump_force = player.get_jump_force();
        player.set_run_speed(run_speed * 2.0);
        player.set_jump_force(jump_force * 1.5);
    }

    /// The low burn function for pewter.
    /// This function gives the player a small speed boost and a small jump boost.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the run speed and jump force can be modified.
    fn low_burn(&mut self) {
        self.player
            .bind_mut()
            .get_pewter_particles()
            .set_visible(true);
        self.update_reserve(-self.low_burn_rate);
        let mut player = self.player.bind_mut();

        let run_speed = player.get_run_speed();
        let jump_force = player.get_jump_force();
        player.set_run_speed(run_speed * 1.5);
        player.set_jump_force(jump_force * 1.2);
    }

    /// The update function for pewter.
    /// This function check to see if the input manager has a pewter event.
    /// If the event is found then the burn function is called.
    /// If the low burn variant is found then the low burn function is called.
    /// This will also toggle the pewter particles on and off.
    fn update(&mut self) {
        let mut input_manager = self.player.bind_mut().get_input_manager();
        self.update_burn(&mut input_manager);
        self.update_low_burn(&mut input_manager);
        if self.current_reserve <= 0.0 {
            self.cleanup_burn();
            self.cleanup_lowburn();
        } else if self.burning {
            self.update_reserve(-self.burn_rate);
        } else if self.low_burning {
            self.update_reserve(-self.low_burn_rate);
        }
    }

    fn update_reserve(&mut self, amount: f64) {
        self.current_reserve += amount;
        self.current_reserve = self.current_reserve.clamp(0.0, self.capacity);
    }

    fn metal_type(&self) -> MetalType {
        self.metal_type
    }

    fn update_burn(&mut self, input_manager: &mut Gd<InputManager>) {
        let mut input_manager = input_manager.bind_mut();
        let burn_type = BurnType::Burn;

        if input_manager.fetch_metal_event((self.metal_type, burn_type, ButtonState::Pressed)) {
            self.burn();
            self.burning = true;
        } else if input_manager.fetch_metal_event((
            self.metal_type,
            burn_type,
            ButtonState::Released,
        )) {
            self.cleanup_burn();
        }
    }

    fn update_low_burn(&mut self, input_manager: &mut Gd<InputManager>) {
        let mut input_manager = input_manager.bind_mut();
        let burn_type = BurnType::LowBurn;

        if input_manager.fetch_metal_event((self.metal_type, burn_type, ButtonState::Pressed)) {
            self.low_burn();
            self.low_burning = true;
        } else if input_manager.fetch_metal_event((
            self.metal_type,
            burn_type,
            ButtonState::Released,
        )) {
            self.cleanup_lowburn();
        }
    }
}
