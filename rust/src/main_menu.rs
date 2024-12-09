use godot::{
    classes::{AnimatedSprite2D, AnimationPlayer, Control, RichTextLabel},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node2D)]

pub struct MainMenu {
    base: Base<Node2D>,
    settings_menu: Option<Gd<Control>>,
    main_menu: Option<Gd<Control>>,
}

#[godot_api]
impl INode2D for MainMenu {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            settings_menu: None,
            main_menu: None,
        }
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
    pub fn remove_player(&self, player_id: i32) {
        self.base()
            .get_node_as::<AnimatedSprite2D>(format!("Player{}", player_id))
            .set_visible(false);
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

    fn get_settings_menu(&mut self) -> Gd<Control> {
        if self.settings_menu.is_none() {
            self.settings_menu = Some(self.base().get_node_as::<Control>("Settings"));
        }

        self.settings_menu
            .as_ref()
            .expect("SettingsMenu node not found")
            .clone()
    }

    fn get_main_menu(&mut self) -> Gd<Control> {
        if self.main_menu.is_none() {
            self.main_menu = Some(self.base().get_node_as::<Control>("MainMenuUI"));
        }

        self.main_menu
            .as_ref()
            .expect("MainMenu node not found")
            .clone()
    }

    #[func]
    pub fn swap_to_settings(&mut self) {
        let mut main_menu = self.get_main_menu();
        let mut settings_menu = self.get_settings_menu();

        main_menu.set_process(false);
        main_menu.set_visible(false);

        settings_menu.set_process(true);
        settings_menu.set_visible(true);
    }

    #[func]
    pub fn swap_to_main_menu(&mut self) {
        let mut main_menu = self.get_main_menu();
        let mut settings_menu = self.get_settings_menu();

        main_menu.set_process(true);
        main_menu.set_visible(true);

        settings_menu.set_process(false);
        settings_menu.set_visible(false);
    }
}
