use godot::{classes::AnimatedSprite2D, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D)]

pub struct MainMenu {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for MainMenu {
    fn init(base: Base<Node2D>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl MainMenu {
    #[func]
    pub fn add_player(&self, player_id: i32) {
        self.base()
            .get_node_as::<AnimatedSprite2D>(format!("Player{}", player_id))
            .set_visible(true);
    }
}
