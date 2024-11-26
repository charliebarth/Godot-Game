extends Control

@onready var game = get_node("/root/Game") as Game

func _on_player_pressed() -> void:
	game.attempt_start(false)


func _on_settings_pressed() -> void:
	pass # Replace with function body.


func _on_quit_pressed() -> void:
	get_tree().quit()


func _on_test_mode_pressed() -> void:
	game.attempt_start(true)
