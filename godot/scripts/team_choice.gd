## Handles the logic for the team choice menu
## 
## @author Trinity Pittman 
## @version Spring 2025
extends Control

## The main menu node
@onready var main_menu = $".." as MainMenu
## The game 
@onready var game = get_node("/root/Game") as Game
## The start game button
@onready var start_game: Button = $"MarginContainer/VBoxContainer/HBoxContainer/Start Game"

## When the team choice menu is visible in the tree, this function recieves 
## input and if the input matches the actions "blue" or "red" we call the main 
## menu and game methods that change the players team to the one selected. 
## @param `event` - The input event that was recieved. 
func _input(event: InputEvent) -> void:
	if self.is_visible_in_tree():
		var device_id = event.device
		if event.is_action("ui_blue"):
			game.set_player_team(device_id, "Blue")
		elif event.is_action("ui_red"):
			game.set_player_team(device_id, "Red")
		

## When the visibility of this scene changes to visible, grab the foucs of a 
## button on screen so controllers can navigate the menu. 
func _on_visibility_changed() -> void:
	if self.is_visible_in_tree():
		start_game.grab_focus()

## When the exit button is pressed, swap to the new game menu (previous menu) 
## and reset players so they are no longer on a team. 
func _on_exit_pressed() -> void:
	main_menu.swap_to_new_game_menu()
	game.reset_team_players()

## Starts the game
func _on_start_game_pressed() -> void:
	game.start_game()

## When the team choice menu exits the tree, stop processing
func _on_tree_exited() -> void:
	set_process(false)

## When the team choice menu enters the tree, start processing
func _on_tree_entered() -> void:
	set_process(true)
	
## When the team choice menu is processed, check if the cancel button is pressed
## If the cancel button is pressed, swap to the new game menu (previous menu)
func _process(_delta: float) -> void:
	if self.visible && Input.is_action_just_pressed("ui_cancel"):
		main_menu.swap_to_new_game_menu()
