extends Control

func _on_player_pressed() -> void:
	var player_manaegr = get_node("/root/Game/PlayerManager") as PlayerManager
	player_manaegr.start()
	get_parent().queue_free()


func _on_settings_pressed() -> void:
	pass # Replace with function body.


func _on_quit_pressed() -> void:
	get_tree().quit()
