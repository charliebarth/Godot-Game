use crate::player::{
    enums::player_states::PlayerStates, player::Player, traits::player_state::PlayerState,
};

#[derive(Clone)]
pub struct CrouchEnd;

impl PlayerState for CrouchEnd {
    fn enter(&self, _player: &mut Player) {}

    fn update(&self, player: &mut Player) {
        if player.is_anim_finished() {
            player.set_state(PlayerStates::Idle);
        }
    }

    fn clone(&self) -> Box<dyn PlayerState> {
        Box::new(CrouchEnd)
    }

    fn as_str(&self, _player: &mut Player) -> &str {
        "crouch_end"
    }
}
