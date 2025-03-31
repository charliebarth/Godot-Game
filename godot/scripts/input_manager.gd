extends InputManager

@onready var game = get_node("/root/Game") as GameGD
@onready var player = get_parent() as Player

func _input(event: InputEvent) -> void:
	var button_name: String = self.get_button_name(event)
	if button_name == "":
		return
		
	var input_data = {
			"player_id": player.get_player_id(),
			"button_name": button_name,
			"is_pressed": event.is_action_pressed(button_name),
			"is_released": event.is_action_released(button_name),
			"action_strength": event.get_action_strength(button_name),
		}
		
	if !multiplayer.is_server():
		game.rpc_id(1, "receive_input", input_data)
	else:
		game.receive_input(input_data)
