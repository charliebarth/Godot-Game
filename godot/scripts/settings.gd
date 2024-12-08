extends Control

@onready var game = get_node("/root/Game") as Game

func _on_exit_pressed() -> void:
	pass # Replace with function body.
	# call return to main menu function on Game


func _on_tree_exited() -> void:
	set_process(false)


func _on_tree_entered() -> void:
	set_process(true)
