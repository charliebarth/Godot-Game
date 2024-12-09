use crate::player::enums::metal_events::MetalEvents;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

pub struct Pewter {
    capacity: f64,
    current_reserve: f64,
    burn_rate: f64,
    low_burn_rate: f64,
    show_particles: bool,
}

impl Pewter {
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
    fn burn(&mut self, player: &mut Player) {
        self.current_reserve -= self.burn_rate;
        player.set_run_speed(player.get_run_speed() * 2.0);
        player.set_jump_force(player.get_jump_force() * 1.5);
    }

    fn low_burn(&mut self, player: &mut Player) {
        self.current_reserve -= self.low_burn_rate;
        player.set_run_speed(player.get_run_speed() * 1.5);
        player.set_jump_force(player.get_jump_force() * 1.2);
    }

    fn update(&mut self, player: &mut Player) {
        let mut godot_input_manager = player.get_input_manager();
        let mut input_manager = godot_input_manager.bind_mut();

        self.show_particles = false;

        if self.current_reserve > 0.0 {
            if input_manager.fetch_metal_event(MetalEvents::Pewter) {
                self.show_particles = true;
                self.burn(player);
            } else if input_manager.fetch_metal_event(MetalEvents::PewterLowBurn) {
                self.low_burn(player);
                self.show_particles = true;
            }
        }

        player.set_metal_reserve_amount(self.as_str().into(), self.current_reserve);
        player
            .get_pewter_particles()
            .set_visible(self.show_particles);
    }

    fn as_str(&self) -> &str {
        "pewter"
    }

    fn increase_reserve(&mut self, amount: f64) {
        self.current_reserve += amount;
        self.current_reserve = self.current_reserve.clamp(0.0, self.capacity);
    }
}
