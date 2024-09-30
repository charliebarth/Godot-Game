use super::{metals::pewter::Pewter, player::Player, traits::metal::Metal};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MetalManager {
    base: Base<Node2D>,
    metals: Vec<Box<dyn Metal>>,
}

#[godot_api]
impl INode2D for MetalManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            metals: Vec::new(),
        }
    }
}

impl MetalManager {
    pub fn assign_starting_metals(&mut self, game_mode: &str) {
        match game_mode {
            "last_player_standing" => self.last_player_standing(),
            _ => {}
        }
    }

    fn last_player_standing(&mut self) {
        self.metals
            .push(Box::new(Pewter::new(100.0, 100.0, 0.05, 0.01)));
    }

    pub fn update(&mut self, player: &mut Player) {
        for metal in &mut self.metals {
            metal.as_mut().update(player);
        }
    }
}
