use godot::{builtin::StringName, classes::Input, obj::WithBaseField};

use crate::player::{
    enums::player_events::PlayerEvents,
    player::{Player, MAX_RUN_SPEED},
    traits::player_state::PlayerState,
};

use super::{crouch_start::CrouchStart, fall::Fall, idle::Idle, jump::Jump, slide::Slide};

#[derive(Clone)]
pub struct Sprint;

impl PlayerState for Sprint {
    fn enter(&self, player: &mut Player) {
        self.run(player);
    }

    fn update(&self, player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();
        let mut input_manager_unbound = player.get_input_manager();
        let mut input_manager = input_manager_unbound.bind_mut();

        if horizontal_dir == 0.0 {
            player.set_state(Box::new(Idle));
        } else if Input::singleton().is_action_just_pressed(StringName::from("jump"))
            && player.base().is_on_floor()
        {
            player.set_state(Box::new(Jump));
        } else if !player.base().is_on_floor() {
            player.set_state(Box::new(Fall));
        // If player attempts to crouch while sprinting they slide into a crouch
        } else if input_manager.fetch_event(PlayerEvents::Crouch) {
            player.set_previous_state(Box::new(CrouchStart));
            player.set_state(Box::new(Slide));
        // If player attempts to roll while sprinting they slide instead
        } else if input_manager.fetch_event(PlayerEvents::Roll) {
            player.set_state(Box::new(Slide));
        } else {
            self.run(player);
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Sprint)
    }

    fn as_str(&self, _player: &mut Player) -> &str {
        "run"
    }
}

impl Sprint {
    fn run(&self, player: &mut Player) {
        player.apply_horizontal_velocity(player.get_dir(), MAX_RUN_SPEED * 1.4);

        player.get_sprite().set_speed_scale(1.4);
    }
}
