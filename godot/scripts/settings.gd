extends Control
@onready var main_menu = $".." as MainMenu

## When the exit button is pressed, swap to the main menu
func _on_exit_pressed() -> void:
	main_menu.swap_to_main_menu()

## When the settings menu exits the tree, stop processing
func _on_tree_exited() -> void:
	set_process(false)

## When the settings menu enters the tree, start processing
func _on_tree_entered() -> void:
	set_process(true)

## When the settings menu is processed, check if the cancel button is pressed
## If the cancel button is pressed, swap to the main menu
func _process(_delta: float) -> void:
	if self.visible && Input.is_action_just_pressed("ui_cancel"):
		main_menu.swap_to_main_menu()
