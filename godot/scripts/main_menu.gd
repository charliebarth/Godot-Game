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
# A reference to the settings button
@onready var settings: Button = $MarginContainer/HBoxContainer/VBoxContainer/Settings
# A reference to the quit button
@onready var quit: Button = $MarginContainer/HBoxContainer/VBoxContainer/Quit
# A reference to the tutorial button
@onready var tutorial: Button = $MarginContainer/HBoxContainer/VBoxContainer/Tutorial

## When the play button is pressed, swap to the new game menu 
func _on_play_pressed() -> void:
	if play.text == "Play":
		main_menu.swap_to_new_game_menu()
	elif play.text == "Local Multiplayer":
		self.swap_to_main_menu()
	elif play.text == "Host":
		game.host()
		self.swap_to_server_main_menu()
	elif play.text == "Ready" || play.text == "Readied":
		game.rpc_id(1, "ready", multiplayer.get_unique_id())
		if play.text == "Ready":
			play.text = "Readied"
		else:
			play.text = "Ready"

## When the settings button is pressed, swap to the settings menu
func _on_settings_pressed() -> void:
	if settings.text == "Settings":
		main_menu.swap_to_settings()
	elif settings.text == "Online Multiplayer":
		self.swap_to_online_mode_select()
	elif settings.text == "Join":
		game.join()
		self.swap_to_client_main_menu()

## When the quit button is pressed, quit the game
func _on_quit_pressed() -> void:
	get_tree().quit()
	
## When the visibility of the main menu is changed, grab the focus
## This allows controllers to navigate the menu
func _on_visibility_changed() -> void:
	if play != null && self.is_visible_in_tree():
		play.grab_focus.call_deferred()
		
## When the main menu is ready, grab the focus
func _on_ready() -> void:
	if play.is_visible_in_tree():
		play.grab_focus.call_deferred()

## When the tutorial button is pressed, start the tutorial
func _on_tutorial_pressed() -> void:
	game.start_tutorial()
	
## Swaps to the online mode select menu
func swap_to_online_mode_select():
	Settings.set_online_multiplayer(true)
	play.text = "Host"
	settings.text = "Join"

## Swaps to the main menu
func swap_to_main_menu():
	Settings.set_online_multiplayer(false)
	game.set_accept_input(true)
	tutorial.visible = true
	play.text = "Play"
	settings.text = "Settings"

## Swaps to the client main menu
func swap_to_client_main_menu():
	play.text = "Ready"
	settings.text = "Settings"

## Swaps to the server main menu
func swap_to_server_main_menu():
	play.text = "Server"
	settings.text = "Server"
