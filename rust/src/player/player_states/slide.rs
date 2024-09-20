use crate::player::{
    player::{Player, MAX_RUN_SPEED},
    traits::player_state::PlayerState,
};

#[derive(Clone)]
pub struct Slide;

impl PlayerState for Slide {
    fn enter(&self, player: &mut Player) {
        player.apply_horizontal_velocity(player.get_dir(), MAX_RUN_SPEED * 1.5);
    }

    fn update(&self, player: &mut Player) {
        if player.is_anim_finished() {
            player.set_state(player.get_previous_state());
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(Slide)
    }

    fn as_str(&self, _player: &mut Player) -> &str {
        "slide"
    }
}
