use godot::classes::{ITextureProgressBar, TextureProgressBar};
/// Represents a coin counter.
///
/// Author : Trinity Pittman
/// Version : 09/18/2024
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TextureProgressBar)]
pub struct CoinCounter {
    base: Base<TextureProgressBar>,
    /// The amount of Metal reserved in the bar
    coins: f64,
}

#[godot_api]
impl ITextureProgressBar for CoinCounter {
    fn init(base: Base<TextureProgressBar>) -> Self {
        Self { base, coins: 0.0 }
    }

    fn ready(&mut self) {
        // do i need to set textures?
    }
}

impl CoinCounter {
    fn add_coin() {
        self.coins += 1;
        // Update coin counter
    }

    fn remove_coins(&mut self, num_coins: f64) -> bool {
        if self.coins - num_coins < 0 {
            false
        } else {
            self.coins -= num_coins;
            true
        }
    }
}
