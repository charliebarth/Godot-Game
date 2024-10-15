use crate::player::{player::Player, traits::player_state::PlayerState};

#[derive(Clone, Copy)]
pub struct Slide;

impl PlayerState for Slide {
    fn enter(player: &mut Player) {
        let dir = player.get_dir();
        let speed = player.get_run_speed() * 1.5;
        player.apply_horizontal_velocity(dir, speed);
    }

    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            let previous_state = player.get_previous_state();
            player.set_state(previous_state);
        }
    }
}
