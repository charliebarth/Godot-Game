extends Control

@onready var main_menu = $".." as MainMenu
@onready var game = get_node("/root/Game") as Game
@onready var start_game: Button = $"MarginContainer/VBoxContainer/HBoxContainer/Start Game"
@onready var player_1: AnimatedSprite2D = $"../Player1"

## When the team choice menu is visible in the tree, this function recieves 
## input and if the input matches the actions "blue" or "red" we call the main 
## menu and game methods that change the players team to the one selected. 
## @param `event` - The input event that was recieved. 
func _input(event: InputEvent) -> void:
	if self.is_visible_in_tree():
		var device_id = event.device
		if event.is_action("blue"):
			main_menu.set_player_team(device_id, true)
			game.set_player_team(device_id, "Blue")
		elif event.is_action("red"):
			main_menu.set_player_team(device_id, false)
			game.set_player_team(device_id, "Red")
		

## When the visibility of this scene changes to visible, grab the foucs of a 
## button on screen so controllers can navigate the menu. 
func _on_visibility_changed() -> void:
	if self.visible:
		start_game.grab_focus()

## When the exit button is pressed, swap to the new game menu (previous menu)
func _on_exit_pressed() -> void:
	main_menu.swap_to_new_game_menu()

func _on_start_game_pressed() -> void:
	#if game.get_number_of_players() == game.
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
