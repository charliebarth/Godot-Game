## Handles logic for the main menu buttons and screen swapping
## 
## @author Charles Barth
## @author Trinity Pittman
extends Control

# A reference to the game
@onready var game = get_node("/root/Game") as Game
# A reference to the main menu (for rust methods)
@onready var main_menu = $".." as MainMenu
# A reference to the play button
@onready var play: Button = $MarginContainer/HBoxContainer/VBoxContainer/Play

## When the play button is pressed, swap to the new game menu 
func _on_play_pressed() -> void:
	main_menu.swap_to_new_game_menu()

## When the settings button is pressed, swap to the settings menu
func _on_settings_pressed() -> void:
	main_menu.swap_to_settings()

## When the quit button is pressed, quit the game
func _on_quit_pressed() -> void:
	get_tree().quit()
	
## When the visibility of the main menu is changed, grab the focus
## This allows controllers to navigate the menu
func _on_visibility_changed() -> void:
	if self.visible && play != null:
		play.grab_focus.call_deferred()
		
## When the main menu is ready, grab the focus
func _on_ready() -> void:
	if self.visible:
		play.grab_focus.call_deferred()
