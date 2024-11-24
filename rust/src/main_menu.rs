use godot::{
    classes::{AnimatedSprite2D, AnimationPlayer, RichTextLabel},
    prelude::*,
};

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

    #[func]
    pub fn add_notification(&self, msg: String) {
        let mut notification_box = self.base().get_node_as::<RichTextLabel>("NotificationBox");
        notification_box.clear();
        notification_box.append_text(format!("[center]{}", msg).into());

        let mut notification_box_animation =
            notification_box.get_node_as::<AnimationPlayer>("AnimationPlayer");
        notification_box_animation.set_current_animation("fade_out".into());
        notification_box_animation.play();
    }
}
