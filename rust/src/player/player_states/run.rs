use godot::{builtin::StringName, classes::Input, obj::WithBaseField};
use std::time::{Duration, Instant};

use crate::player::{player::Player, traits::player_state::PlayerState};

use super::{crouch::Crouch, fall::Fall, idle::Idle, jump::Jump, roll::Roll};

const MAX_RUN_SPEED: f32 = 140.0;

#[derive(Clone)]
pub struct Run;

impl PlayerState for Run {
    fn enter(&self, player: &mut Player) {
        self.run(player);
    }

    fn update(&self, player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();
        let b_button = StringName::from("b_button");

        if horizontal_dir == 0.0 {
            player.set_state(Box::new(Idle));
        } else if Input::singleton().is_action_just_pressed(StringName::from("jump"))
            && player.base().is_on_floor()
        {
            player.set_state(Box::new(Jump));
        } else if !player.base().is_on_floor() {
            player.set_state(Box::new(Fall));
        } else if Input::singleton().is_action_just_released(b_button.clone()) {
            if let Some(start_time) = player.get_button_press_time(&b_button) {
                let duration_held = start_time.elapsed();
                if duration_held < Duration::from_millis(300) {
                    player.set_state(Box::new(Roll)); // If held less than 300ms, roll
                } else {
                    player.set_state(Box::new(Crouch)); // If held for 300ms or more, crouch
                }

                // Remove the press time after processing using the remover
                player.remove_button_press_time(&b_button);
            }
        } else if Input::singleton().is_action_just_pressed(b_button.clone()) {
            player.set_button_press_time(b_button.clone(), Instant::now());
        } else {
            self.run(player);
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Run)
    }

    fn as_str(&self, player: &mut Player) -> &str {
        let horizontal_dir = player.get_horizontal_movement();

        if horizontal_dir.abs() > 0.3 {
            "run"
        } else {
            // "walk"
            // TODO: Implement walk animation
            "run"
        }
    }
}

impl Run {
    fn run(&self, player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            return;
        }

        player.set_dir(horizontal_dir);
        player.apply_horizontal_velocity(horizontal_dir, MAX_RUN_SPEED);

        // TODO: Rewrite this and as_str to swap between run and walk
        let animation_speed = if horizontal_dir.abs() < 0.3 {
            0.3
        } else {
            horizontal_dir.abs()
        };

        player.get_sprite().set_speed_scale(animation_speed);
    }
}
