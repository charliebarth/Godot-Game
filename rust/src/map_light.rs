use godot::{
    classes::{IPointLight2D, PointLight2D, Tween},
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
    pub fn transition_light_levels(&mut self, light_level: f32, transition_time: f64, scale: f32) {
        let target_energy = light_level * self.energy;
        let target_scale = scale * self.scale;
        let target_scale_vec = Vector2::new(target_scale, target_scale);

        let mut tween = self
            .base_mut()
            .create_tween()
            .expect("Failed to create tween");

        tween.tween_property(
            &self.base_mut().get_node_as::<MapLight>("."),
            "energy",
            &Variant::from(target_energy),
            transition_time,
        );

        let mut tween = self
            .base_mut()
            .create_tween()
            .expect("Failed to create tween");

        tween.tween_property(
            &self.base_mut().get_node_as::<MapLight>("."),
            "scale",
            &Variant::from(target_scale_vec),
            transition_time,
        );
    }
}
