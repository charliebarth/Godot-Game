extends Control

@onready var map_scene = preload("res://scenes/game.tscn") as PackedScene

func _on_player_pressed() -> void:
	get_tree().change_scene_to_packed(map_scene)


func _on_settings_pressed() -> void:
	pass # Replace with function body.


func _on_quit_pressed() -> void:
	get_tree().quit()
