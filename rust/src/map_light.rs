use std::default;

use godot::{
    classes::{IPointLight2D, PointLight2D},
    prelude::*,
};

use crate::game::Game;

#[derive(GodotClass)]
#[class(base=PointLight2D)]
/// Represents a light in the game
pub struct MapLight {
    /// The base node of the MapLight
    base: Base<PointLight2D>,
    /// The default energy of the light
    energy: f32,
    scale: f32,
}

#[godot_api]
impl IPointLight2D for MapLight {
    fn init(base: Base<PointLight2D>) -> Self {
        Self {
            base,
            energy: 1.0,
            scale: 1.0,
        }
    }

    fn ready(&mut self) {
        self.energy = self.base().get_energy();
        self.scale = self.base().get_scale().x;

        let mut game = self.base().get_node_as::<Game>("/root/Game");
        let light = self.base().get_node_as::<MapLight>(".");
        game.connect(
            "change_cycle_map",
            &Callable::from_object_method(&light, "transition_light_levels"),
        );
    }
}

#[godot_api]
impl MapLight {
    #[func]
    pub fn transition_light_levels(&mut self, light_level: f32, _transition_time: f64, scale: f32) {
        let default_energy = self.energy;
        self.base_mut().set_energy(light_level * default_energy);

        let default_scale = self.scale;
        let new_scale = scale * default_scale;
        self.base_mut()
            .set_scale(Vector2::new(new_scale, new_scale));
    }
}
