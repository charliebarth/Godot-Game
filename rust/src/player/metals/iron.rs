// iron.rs
//
// This file contains the implementation of the Iron player ability.
// The Iron ability allows players to pull metal objects toward them.
// This ability is useful for damaging other player via pulling metal objects into other player,
// but also for moving around the map.
//
// Author: Charles Barth
// Version: Spring 2025
use godot::obj::Gd;

use crate::player::{enums::metal_type::MetalType, player::Player, traits::metal::Metal};

use super::steel::Steel;

/// The initial burn direction for the Iron ability.
const PULL_BURN_DIRECTION: f32 = -1.0;

pub struct Iron {
    steel: Steel,
}

impl Iron {
    pub fn new(
        capacity: f64,
        current_reserve: f64,
        burn_rate: f64,
        low_burn_rate: f64,
        player: Gd<Player>,
        metal_type: MetalType,
    ) -> Self {
        let mut steel = Steel::new(
            capacity,
            current_reserve,
            burn_rate,
            low_burn_rate,
            player,
            metal_type,
        );
        steel.set_burn_direction(PULL_BURN_DIRECTION);
        Iron { steel }
    }
}

impl Metal for Iron {
    fn burn(&mut self) {
        self.steel.burn();
    }

    fn low_burn(&mut self) {
        self.steel.low_burn();
    }

    fn update_reserve(&mut self, amount: f64) {
        self.steel.update_reserve(amount);
    }

    fn metal_type(&self) -> MetalType {
        self.steel.metal_type()
    }

    fn burning(&self) -> bool {
        self.steel.burning()
    }

    fn current_reserve(&self) -> f64 {
        self.steel.current_reserve()
    }

    fn get_player(&mut self) -> godot::prelude::GdMut<'_, Player> {
        self.steel.get_player()
    }

    fn low_burning(&self) -> bool {
        self.steel.low_burning()
    }

    fn previous_reserve(&self) -> f64 {
        self.steel.previous_reserve()
    }

    fn set_burning(&mut self, burning: bool) {
        self.steel.set_burning(burning);
    }

    fn set_low_burning(&mut self, low_burning: bool) {
        self.steel.set_low_burning(low_burning);
    }

    fn set_previous_reserve(&mut self, amt: f64) {
        self.steel.set_previous_reserve(amt);
    }
}
