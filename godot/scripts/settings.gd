extends Control
@onready var main_menu = $".." as MainMenu

func _on_exit_pressed() -> void:
	main_menu.swap_to_main_menu()


func _on_tree_exited() -> void:
	set_process(false)


func _on_tree_entered() -> void:
	set_process(true)
