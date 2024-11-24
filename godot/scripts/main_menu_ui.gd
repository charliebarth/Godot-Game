extends Control

@onready var game = get_node("/root/Game") as Game

func _on_player_pressed() -> void:
	var attempt_start = game.attempt_start(false)
	if attempt_start:
		game.start_game()
	else:
		var main_menu = get_parent() as MainMenu
		main_menu.add_notification("Too few players connected")


func _on_settings_pressed() -> void:
	pass # Replace with function body.


func _on_quit_pressed() -> void:
	get_tree().quit()
