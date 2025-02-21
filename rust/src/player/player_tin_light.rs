use godot::{
    classes::{IPointLight2D, PointLight2D},
    prelude::*,
};

use crate::game::Game;

#[derive(GodotClass)]
#[class(base=PointLight2D)]
/// Represents a light in the game
pub struct PlayerTinLight {
    /// The base node of the PlayerLight
    base: Base<PointLight2D>,
    /// The default energy of the light
    energy: f32,
}

#[godot_api]
impl IPointLight2D for PlayerTinLight {
    fn init(base: Base<PointLight2D>) -> Self {
        Self { base, energy: 1.0 }
    }

    fn ready(&mut self) {
        self.energy = self.base().get_energy();

        // let mut game = self.base().get_node_as::<Game>("/root/Game");
        // let light = self.base().get_node_as::<PlayerTinLight>(".");
        // game.connect(
        //     "change_cycle_player",
        //     &Callable::from_object_method(&light, "transition_light_levels"),
        // );
    }
}

#[godot_api]
impl PlayerTinLight {
    /// Adjusts the light level of the player's tin light.
    ///
    /// # Arguments
    /// * `light_level` - The target light level.
    /// * `transition_time` - The time it takes to transition to the target light level.
    #[func]
    pub fn adjust_tin_light(&mut self, light_level: f32, transition_time: f64) {
        let target_energy = light_level * self.energy;

        let mut tween = self
            .base_mut()
            .create_tween()
            .expect("Failed to create tween");

        tween.tween_property(
            &self.base_mut().get_node_as::<PlayerTinLight>("."),
            "energy",
            &Variant::from(target_energy),
            transition_time,
        );
    }
}
