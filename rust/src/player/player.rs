use godot::classes::AnimatedSprite2D;
use godot::classes::Area2D;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::classes::ProjectSettings;
use godot::classes::TextureProgressBar;
use godot::classes::Label;
use godot::prelude::*;

use crate::ui::coin_counter::CoinCounter;
use crate::ui::metal_reserve_bar_manager::MetalReserveBarManager;

use super::input_manager::InputManager;
use super::player_states::idle::Idle;
use super::traits::player_state::PlayerState;

const MAX_HEALTH: f64 = 100.;
const MIN_HEALTH: f64 = 0.;
pub const MAX_RUN_SPEED: f32 = 140.0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    direction: f32,
    gravity: f64,
    health: f64,
    delta: f64,
    current_state: Box<dyn PlayerState>,
    previous_state: Box<dyn PlayerState>,
    anim_finished: bool,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        let path = GString::from("physics/2d/default_gravity");
        let gravity: f64 =
            FromGodot::try_from_variant(&ProjectSettings::singleton().get_setting(path)).unwrap();

        Self {
            base,
            current_state: Box::new(Idle),
            previous_state: Box::new(Idle),
            direction: 1.0,
            health: 100.,
            delta: 0.0,
            gravity,
            anim_finished: false,
        }
    }

    fn ready(&mut self) {
        self.set_state(Box::new(Idle));
    }

    fn physics_process(&mut self, delta: f64) {
        self.set_delta(delta);

        let mut base_vel = self.base_mut().get_velocity();

        let sprite = self.get_sprite();

        if !sprite.is_playing() {
            self.set_anim_finished();
        }

        if !self.base().is_on_floor() {
            base_vel.y += (self.gravity * self.delta) as f32;
        } else {
            base_vel.y = 0.0;
        }

        self.base_mut().set_velocity(base_vel);

        let mut sprite: Gd<AnimatedSprite2D> = self.get_sprite();
        sprite.set_speed_scale(1.0);

        self.get_current_state().update(self);
        self.update_animation();

        self.base_mut().move_and_slide();
    }
}

impl Player {
    pub fn set_state(&mut self, new_state: Box<dyn PlayerState>) {
        self.previous_state = self.get_current_state();
        self.current_state = new_state;
        self.get_current_state().enter(self);
    }

    pub fn get_current_state(&self) -> Box<dyn PlayerState> {
        self.current_state.clone()
    }

    fn set_delta(&mut self, delta: f64) {
        self.delta = delta;
    }

    pub fn get_delta(&self) -> f64 {
        self.delta
    }

    pub fn get_health(&self) -> f64 {
        self.health
    }

    pub fn get_dir(&self) -> f32 {
        self.direction
    }

    pub fn set_dir(&mut self, direction: f32) {
        if direction < 0.0 {
            self.direction = -1.0;
        } else if direction > 0.0 {
            self.direction = 1.0;
        }
    }

    pub fn adjust_health(&mut self, health: f64) {
        // Adjust health positively or negatively
        let new_health: f64 = if health < 0. {
            // Subtract health, but ensure we handle underflow
            if self.health < -health {
                MIN_HEALTH
            } else {
                self.health + health
            }
        } else {
            // Add health, but ensure no overflow
            if self.health + health > MAX_HEALTH {
                MAX_HEALTH
            } else {
                self.health + health
            }
        };

        // Clamp health between MIN_HEALTH and MAX_HEALTH
        self.health = new_health.clamp(MIN_HEALTH, MAX_HEALTH);

        // Find the health bar then change the health value 
        let children: Array<Gd<Node>> = self.base.to_gd().get_children();
        for i in 0..children.len() {
            let child : Gd<Node> = children.get(i).expect("");
            
            if child.get_name().to_string() == "HealthBar" {
                let mut health_bar = child.cast::<TextureProgressBar>();
                // Set the health value 
                health_bar.set_value(self.health); 
    
            }
        }
    }

    /// Adjusts the coins in this players coin_counter positively or negatively. 
    ///
    /// Args:
    ///     pos_neg (i8): if -1, remove_coin    if +1, add_coin
    pub fn adjust_coins(&mut self, pos_neg: i8) {
        // Get all the children of the player 
        let children: Array<Gd<Node>> = self.base.to_gd().get_children(); 
        for i in 0..children.len() { 
            // Go through the children and find the `Coin_Counter_Panel`
            let child : Gd<Node> = children.get(i).expect("");
            if child.get_name().to_string() == "Coin_Counter_Panel" {
                // Get all the children of the Coin_Counter_Panel
                let children_counter: Array<Gd<Node>> = child.get_children();
                for j in 0..children_counter.len() {
                    // Go through the Coin_Counter_Panel to find `CoinCounter`
                    let child_counter : Gd<Node> = children_counter.get(j).expect("");
                    if child_counter.get_name().to_string() == "CoinCounter" {

                        // Cast the child to CoinCounter and call `add_coin` or `remove_coin`
                        if let Ok(mut coin_label) = 
                                                        child_counter.try_cast::<CoinCounter>() {
                            if pos_neg == -1 {  // Dereference and call the method
                                coin_label.bind_mut().remove_coin(); 
                            } else {
                                coin_label.bind_mut().add_coin(); 
                            }
                        } else {
                            godot_print!("Failed to cast node to CoinCounter");
                        }
                    }
                }
            }
        }
    }

    /// Adjusts the metals in this players metal bar manager positively.
    pub fn adjust_metals(&mut self) {
        // Get all the children of the player 
        let children: Array<Gd<Node>> = self.base.to_gd().get_children(); 
        for i in 0..children.len() { 
            // Go through the children and find the `metal_reserver_bar_manager`
            let child : Gd<Node> = children.get(i).expect("");
            if child.get_name().to_string() == "MetalReserveBarManager" {
                if let Ok(mut metal_manager) = 
                                        child.try_cast::<MetalReserveBarManager>() {
                    metal_manager.bind_mut().add_metals();
                } else {
                    godot_print!("Failed to cast node to CoinCounter");
                }
            }
        }
    }

    /// Represents the direction the player is trying to move
    /// Returns 1 when the move right button is pressed, -1 when the move left button is pressed, and 0 if neither is pressed
    // TODO: Rename
    pub fn get_horizontal_movement(&mut self) -> f32 {
        let move_left = StringName::from("move_left");
        let move_right = StringName::from("move_right");
        Input::singleton().get_axis(move_left, move_right)
    }

    pub fn apply_horizontal_velocity(&mut self, direction: f32, max_speed: f32) {
        let mut base = self.base_mut();
        let mut base_vel = base.get_velocity();
        base_vel.x = max_speed * direction;
        base.set_velocity(base_vel);
    }

    pub fn set_anim_finished(&mut self) {
        self.anim_finished = true;
    }

    pub fn is_anim_finished(&self) -> bool {
        self.anim_finished
    }

    pub fn get_gravity(&self) -> f64 {
        self.gravity
    }

    fn update_animation(&mut self) {
        let mut sprite = self.get_sprite();

        self.set_animation_direction(&mut sprite);

        let animation_name = StringName::from(self.get_current_state().as_str(self));
        if sprite.get_animation() != animation_name {
            self.anim_finished = false;
            sprite.set_animation(animation_name.into());
            sprite.play();
        }
    }

    fn set_animation_direction(&mut self, sprite: &mut Gd<AnimatedSprite2D>) {
        let mut scale = sprite.get_scale();
        let mut pos = sprite.get_position();

        if self.direction < 0.0 && scale.x != -1.3 {
            scale.x = -1.3;
            pos.x -= 9.0;
        } else if self.direction > 0.0 && scale.x != 1.3 {
            scale.x = 1.3;
            pos.x += 9.0;
        }

        sprite.set_scale(scale);
        sprite.set_position(pos);
    }

    pub fn get_sprite(&self) -> Gd<AnimatedSprite2D> {
        self.base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D")
    }

    pub fn get_previous_state(&self) -> Box<dyn PlayerState> {
        self.previous_state.clone()
    }

    pub fn set_previous_state(&mut self, state: Box<dyn PlayerState>) {
        self.previous_state = state;
    }

    pub fn get_input_manager(&self) -> Gd<InputManager> {
        self.base().get_node_as::<InputManager>("InputManager")
    }
}
