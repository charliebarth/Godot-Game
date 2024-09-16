use godot::classes::AnimatedSprite2D;
use godot::{
    classes::Input,
    global::godot_print,
    obj::{Gd, WithBaseField},
    prelude::StringName,
};

use crate::player::{player::Player, traits::player_state::PlayerState};

use super::{fall::Fall, idle::Idle};

const CROUCH_SPEED: f32 = 75.0;

#[derive(Clone)]
pub struct Crouch;

impl PlayerState for Crouch {
    fn enter(&self, _player: &mut Player) {}

    fn update(&self, player: &mut Player) {
        let animation_name = self.as_str(player);
        godot_print!("Crouch Animation name: {}", animation_name);

        if !player.base().is_on_floor() {
            player.set_state(Box::new(Fall));
        } else if animation_name == "crouch_walk" {
            self.run(player, animation_name);
        } else if animation_name == "crouch_end" && player.is_anim_finished() {
            player.set_state(Box::new(Idle));
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Crouch)
    }

    fn as_str(&self, player: &mut Player) -> &str {
        let sprite: Gd<AnimatedSprite2D> = player.get_sprite();
        let animation_name: StringName = sprite.get_animation();
        let animation_name_str: String = animation_name.to_string();
        let is_crouching = animation_name_str.contains("crouch");

        if !is_crouching {
            return "crouch_start";
        } else if is_crouching && Input::singleton().is_action_pressed("crouch".into()) {
            return "crouch_end";
        } else {
            return "crouch_walk";
        }
    }
}

impl Crouch {
    fn run(&self, player: &mut Player, animation_name: &str) {
        let horizontal_dir = player.get_horizontal_movement();

        player.set_dir(horizontal_dir);
        player.apply_horizontal_velocity(horizontal_dir, CROUCH_SPEED);

        if animation_name == "crouch_walk" && horizontal_dir == 0.0 {
            player.get_sprite().pause();
        } else {
            player.get_sprite().play();
        }
    }
}
