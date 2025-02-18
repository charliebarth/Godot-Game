use crate::player::enums::metal_type::MetalEvents;
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
    /// A flag to determine if the player should show the pewter particles.
    show_particles: bool,
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
    pub fn new(capacity: f64, current_reserve: f64, burn_rate: f64, low_burn_rate: f64) -> Self {
        Self {
            capacity,
            current_reserve,
            burn_rate,
            low_burn_rate,
            show_particles: false,
        }
    }
}

impl Metal for Pewter {
    /// The burn function for pewter.
    /// This function gives the player a large speed boost and a large jump boost.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the run speed and jump force can be modified.
    fn burn(&mut self) {
        // self.current_reserve -= self.burn_rate;
        // player.set_run_speed(player.get_run_speed() * 2.0);
        // player.set_jump_force(player.get_jump_force() * 1.5);
    }

    /// The low burn function for pewter.
    /// This function gives the player a small speed boost and a small jump boost.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the run speed and jump force can be modified.
    fn low_burn(&mut self) {
        // self.current_reserve -= self.low_burn_rate;
        // player.set_run_speed(player.get_run_speed() * 1.5);
        // player.set_jump_force(player.get_jump_force() * 1.2);
    }

    // /// The update function for pewter.
    // /// This function check to see if the input manager has a pewter event.
    // /// If the event is found then the burn function is called.
    // /// If the low burn variant is found then the low burn function is called.
    // /// This will also toggle the pewter particles on and off.
    // ///
    // /// # Arguments
    // /// * `player` - A mutable reference to the player so that the input manager can be accessed.
    // fn update(&mut self, player: &mut Player) {
    //     let mut godot_input_manager = player.get_input_manager();
    //     let mut input_manager = godot_input_manager.bind_mut();

    //     self.show_particles = false;

    //     if self.current_reserve > 0.0 {
    //         if input_manager.fetch_metal_event(MetalEvents::Pewter) {
    //             self.show_particles = true;
    //             self.burn(player);
    //         } else if input_manager.fetch_metal_event(MetalEvents::PewterLowBurn) {
    //             self.low_burn(player);
    //             self.show_particles = true;
    //         }
    //     }

    //     player.set_metal_reserve_amount(self.as_str().into(), self.current_reserve);
    //     player
    //         .get_pewter_particles()
    //         .set_visible(self.show_particles);
    // }

    fn as_str(&self) -> &str {
        "pewter"
    }

    fn increase_reserve(&mut self, amount: f64) {
        self.current_reserve += amount;
        self.current_reserve = self.current_reserve.clamp(0.0, self.capacity);
    }
}
