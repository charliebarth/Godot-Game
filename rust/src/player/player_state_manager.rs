use std::collections::HashMap;

use godot::prelude::*;

use super::{
    enums::player_states::PlayerStates,
    player::Player,
    player_states::{
        crouch::Crouch, crouch_end::CrouchEnd, crouch_start::CrouchStart, fall::Fall, idle::Idle,
        jump::Jump, land::Land, roll::Roll, run::Run, slide::Slide, sprint::Sprint,
        wall_slide::WallSlide,
    },
    traits::player_state::PlayerState,
};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PlayerStateManager {
    base: Base<Node2D>,
    player_states: HashMap<PlayerStates, Box<dyn PlayerState>>,
}

#[godot_api]
impl INode2D for PlayerStateManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            player_states: HashMap::new(),
        }
    }

    fn ready(&mut self) {
        self.fill_player_states();
    }
}

impl PlayerStateManager {
    pub fn get_state(&self, state: PlayerStates) -> &Box<dyn PlayerState> {
        self.player_states.get(&state).unwrap()
    }

    fn fill_player_states(&mut self) {
        self.player_states
            .insert(PlayerStates::Idle, Box::new(Idle));
        self.player_states
            .insert(PlayerStates::Crouch, Box::new(Crouch));
        self.player_states
            .insert(PlayerStates::CrouchEnd, Box::new(CrouchEnd));
        self.player_states
            .insert(PlayerStates::CrouchStart, Box::new(CrouchStart));
        self.player_states
            .insert(PlayerStates::Fall, Box::new(Fall));
        self.player_states
            .insert(PlayerStates::Jump, Box::new(Jump));
        self.player_states
            .insert(PlayerStates::Land, Box::new(Land));
        self.player_states
            .insert(PlayerStates::Roll, Box::new(Roll));
        self.player_states.insert(PlayerStates::Run, Box::new(Run));
        self.player_states
            .insert(PlayerStates::Slide, Box::new(Slide));
        self.player_states
            .insert(PlayerStates::Sprint, Box::new(Sprint));
        self.player_states
            .insert(PlayerStates::WallSlide, Box::new(WallSlide));
    }

    pub fn enter_state(&mut self, state: PlayerStates, player: &mut Player) {
        self.get_state(state).enter(player);
    }

    pub fn update_state(&mut self, state: PlayerStates, player: &mut Player) {
        self.get_state(state).update(player);
    }
}
