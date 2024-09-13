use godot::obj::WithBaseField;

use crate::player::{player::Player, player_states::land::Land, traits::player_state::PlayerState};

#[derive(Clone)]
pub struct Fall;

impl PlayerState for Fall {
    fn enter(&self, player: &mut Player) {
        let mut base_vel = player.base_mut().get_velocity();
        base_vel.y += (player.get_gravity() * player.get_delta()) as f32;
        player.base_mut().set_velocity(base_vel);
    }

    fn update(&self, player: &mut Player) {
        if player.base().is_on_floor() {
            player.set_state(Box::new(Land));
        } else {
            player.set_state(Box::new(Fall));
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Fall)
    }
}
