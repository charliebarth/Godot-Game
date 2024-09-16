use godot::obj::WithBaseField;

use crate::player::{
    player::{Player, MAX_RUN_SPEED},
    player_states::land::Land,
    traits::player_state::PlayerState,
};

#[derive(Clone)]
pub struct Fall;

impl PlayerState for Fall {
    fn enter(&self, _player: &mut Player) {}

    fn update(&self, player: &mut Player) {
        if player.base().is_on_floor() {
            player.set_state(Box::new(Land));
        } else {
            self.run(player);
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Fall)
    }

    fn as_str(&self, _player: &mut Player) -> &str {
        "fall"
    }
}

impl Fall {
    fn run(&self, player: &mut Player) {
        let horizontal_dir = player.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            return;
        }

        player.set_dir(horizontal_dir);
        player.apply_horizontal_velocity(horizontal_dir, MAX_RUN_SPEED / 2.0);

        // TODO: Rewrite this and as_str to swap between run and walk
        let animation_speed = if horizontal_dir.abs() < 0.3 {
            0.3
        } else {
            horizontal_dir.abs()
        };
    }
}
