use crate::player::{player::Player, traits::player_state::PlayerState};

#[derive(Clone)]
pub struct Roll;

impl PlayerState for Roll {
    fn enter(&self, player: &mut Player) {
        player.get_sprite().set_speed_scale(1.0);
    }

    fn update(&self, player: &mut Player) {
        if player.is_anim_finished() {
            player.set_state(player.get_previous_state());
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Roll)
    }

    fn as_str(&self, _player: &mut Player) -> &str {
        "roll"
    }
}
