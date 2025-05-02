extends InputManager

@onready var game = get_node("/root/Game") as GameGD
@onready var player = get_parent() as Player

func _input(event: InputEvent) -> void:
	if (self.get_device_id() != -1 && self.get_device_id() != event.device) || event.is_echo() || player.is_remote_player():
		return
		
	var button_name: String = self.get_button_name(event)
	if button_name == "":
		return
		
	
	if !Settings.get_online_multiplayer():
		self.handle_input(button_name, event.is_action_pressed(button_name), event.is_action_released(button_name), event.get_action_strength(button_name))
	else:
		var input_data = {
			"player_id": player.get_player_id(),
			"button_name": button_name,
			"is_pressed": event.is_action_pressed(button_name),
			"is_released": event.is_action_released(button_name),
			"action_strength": event.get_action_strength(button_name),
		}

		game.rpc_id(1, "receive_input", input_data)
