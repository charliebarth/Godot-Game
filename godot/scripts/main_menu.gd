extends Control

@onready var game = get_node("/root/Game") as Game
@onready var main_menu = $".." as MainMenu


func _on_play_pressed() -> void:
	game.attempt_start(false)


func _on_test_mode_pressed() -> void:
	game.attempt_start(true)


func _on_settings_pressed() -> void:
	main_menu.swap_to_settings()


func _on_quit_pressed() -> void:
	get_tree().quit()
