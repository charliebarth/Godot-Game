extends Control

@onready var main_menu = $".." as MainMenu
@onready var game = get_node("/root/Game") as Game
@onready var start_game: Button = $"MarginContainer/VBoxContainer/HBoxContainer/Start Game"

## When the visibility of the new game menu changes, grab the focus on the start game button
func _on_visibility_changed() -> void:
	if self.visible:
		start_game.grab_focus()

## When the exit button is pressed, swap to the main menu
func _on_exit_pressed() -> void:
	main_menu.swap_to_main_menu()

## When the start game button is pressed, start the game
func _on_start_game_pressed() -> void:
	if game.get_team_game():
		main_menu.swap_to_team_choice_menu()
	else:
		game.start_game()

## When the new game menu exits the tree, stop processing
func _on_tree_exited() -> void:
	set_process(false)

## When the new game menu enters the tree, start processing
func _on_tree_entered() -> void:
	set_process(true)

## When the new game menu is processed, check if the cancel button is pressed
## If the cancel button is pressed, swap to the main menu
func _process(_delta: float) -> void:
	if self.visible && Input.is_action_just_pressed("ui_cancel"):
		main_menu.swap_to_main_menu()
