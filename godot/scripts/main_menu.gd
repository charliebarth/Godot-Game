extends Control

@onready var game = get_node("/root/Game") as Game
@onready var main_menu = $".." as MainMenu
@onready var play: Button = $MarginContainer/HBoxContainer/VBoxContainer/Play

## When the play button is pressed, attempt to start the game
func _on_play_pressed() -> void:
	#main_menu.swap_to_new_game_menu()
	game.start_game()

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
