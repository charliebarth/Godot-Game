use godot::classes::{Area2D, IArea2D};
/// Represents a coin.
///
/// Author : Trinity Pittman
/// Version : 09/21/2024
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TextureProgressBar)]
pub struct Coin {
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Coin {
    fn init(base: Base<Area2D>) -> Self {
        Self { base }
    }
}

impl Coin {}
