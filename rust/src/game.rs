use std::collections::HashMap;

use godot::prelude::*;

use crate::{main_menu::MainMenu, player_manager::PlayerManager};

#[derive(GodotClass)]
#[class(base=Node2D)]

pub struct Game {
    base: Base<Node2D>,
    maps: HashMap<String, Gd<PackedScene>>,
    main_menu: Option<Gd<MainMenu>>,
    player_manager: Option<Gd<PlayerManager>>,
    // game_manager: Gd<GameManager>,
}

#[godot_api]
impl INode2D for Game {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            maps: HashMap::new(),
            main_menu: None,
            player_manager: None,
        }
    }

    fn ready(&mut self) {
        let map_one = load::<PackedScene>("res://scenes/map_one.tscn");
        self.maps.insert("MapOne".to_string(), map_one);
    }
}

#[godot_api]
impl Game {
    fn get_main_menu(&mut self) -> Gd<MainMenu> {
        if self.main_menu.is_none() {
            self.main_menu = Some(self.base().get_node_as::<MainMenu>("MainMenu"));
        }

        self.main_menu
            .as_ref()
            .expect("MetalLine node not found")
            .clone()
    }

    fn get_player_manager(&mut self) -> Gd<PlayerManager> {
        if self.player_manager.is_none() {
            self.player_manager = Some(self.base().get_node_as::<PlayerManager>("PlayerManager"));
        }

        self.player_manager
            .as_ref()
            .expect("MetalLine node not found")
            .clone()
    }

    #[func]
    pub fn attempt_start(&mut self) -> bool {
        let num_players = self.get_player_manager().bind().get_number_of_players();

        if num_players >= 2 {
            return true;
        }

        false
    }

    #[func]
    pub fn start_game(&mut self) {
        // First remove the main menu
        let main_menu = self.get_main_menu();
        self.base_mut().remove_child(main_menu);

        // Next instantiate the map, add it to the tree, and trigger it's camera pan (might make the map auto camera pan on ready/load)
        // Note: Currently defaulting to MapOne
        let map = self
            .maps
            .get("MapOne")
            .expect("Map not found")
            .instantiate_as::<Node2D>();

        // TODO: Call map method to trigger a camera pan accross the map. Once that method has returned start the game.
        // The player manager will reparent the map under the first subviewport and will remove it and all it's children at the end of the game.
        let mut player_manager = self.get_player_manager();
        let mut player_manager = player_manager.bind_mut();
        player_manager.set_map(map);
        player_manager.start();
    }

    #[func]
    pub fn end_game(&mut self, winner: i32, _num_players: i32) {
        // Show a winner screen
        // Next add the main menu back
        let mut main_menu = self.get_main_menu();
        self.base_mut().add_child(main_menu.clone());
        main_menu
            .bind_mut()
            .add_notification(format!("Player {} wins!", winner));
    }
}
