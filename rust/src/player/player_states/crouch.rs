use godot::obj::WithBaseField;

use crate::player::enums::player_events::PlayerEvents;
use crate::player::{player::Player, traits::player_state::PlayerState};

use super::crouch_end::CrouchEnd;
use super::fall::Fall;
use super::roll::Roll;
use super::run::Run;

const CROUCH_SPEED: f32 = 75.0;

#[derive(Clone)]
pub struct Crouch;

impl PlayerState for Crouch {
    fn enter(&self, _player: &mut Player) {}

    fn update(&self, player: &mut Player) {
        let mut input_manager_unbound = player.get_input_manager();
        let mut input_manager = input_manager_unbound.bind_mut();

        if input_manager.fetch_event(PlayerEvents::Crouch) {
            player.set_state(Box::new(CrouchEnd));
        } else if !player.base().is_on_floor() {
            player.set_state(Box::new(Fall));
        } else if input_manager.fetch_event(PlayerEvents::Roll) {
            if player.get_horizontal_movement() != 0.0 {
                player.set_state(Box::new(Roll));
            } else {
                player.set_state(Box::new(CrouchEnd));
            }
        } else if input_manager.fetch_event(PlayerEvents::Sprint) {
            player.set_state(Box::new(Run));
        } else {
            self.run(player);
        }
    }
    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Crouch)
    }

    fn as_str(&self, _player: &mut Player) -> &str {
        "crouch_walk"
    }
}

impl Crouch {
    fn run(&self, player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        player.set_dir(horizontal_dir);
        player.apply_horizontal_velocity(horizontal_dir, CROUCH_SPEED);

        let animation_speed = if horizontal_dir == 0.0 || horizontal_dir.abs() > 0.5 {
            horizontal_dir.abs()
        } else {
            0.5
        };

        player.get_sprite().set_speed_scale(animation_speed);
    }
}
