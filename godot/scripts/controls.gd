extends TabBar

class_name Controls

@onready var h_box_container: HBoxContainer = $"MarginContainer/ScrollContainer/VBoxContainer/Player Choice/HBoxContainer"
@onready var v_box_container: VBoxContainer = $MarginContainer/ScrollContainer/VBoxContainer

var button_group = ButtonGroup.new()

func _ready():
	# Set up Player Button Group 
	for button in h_box_container.get_children():
		if button is Button:
			button.button_group = button_group
	
	button_group.connect("pressed", on_button_press)	
	
	set_keybinds_from_config()
	
	# Load Keybinds for Player 1 on startup 
	load_keybinds_for_device(0)
	
func set_keybinds_from_config():
	var keybind_settings = ConfigFileHandler.load_all_keybind_settings()
	print("SETTINGS  ", keybind_settings)
	# TODO Set settings based on what was loaded 
	
	var actions = ["jump", "sprint", "roll", "attack", "throw", "low_burn", 
				"pewter", "iron", "steel"]
	# Erase current bindings
	for action in actions:
		InputMap.action_erase_events(action)
		
	# Add keybinding for each player 
	for player in keybind_settings: 
		for action in player: 
			InputMap.action_add_event(action, player[action])
			
	
func load_keybinds_for_device(id: int):
	var keybind_settings = load_keybind_settings(id)
	print(keybind_settings, "\n")
	
	for el in v_box_container.get_children():
		if el.has_method("set_button_text"):
			var latest_event = get_action_event_keybound(
				el.action_name, 
				button_group.get_pressed_button().name.to_int() - 1)
			print("LATEST EV: ", latest_event)
			el.set_button_text(keybind_settings, latest_event)


## Gets the keybind settings for a player based on id 
## 
## id (int) - Represents the id of a player (Player 1 has an id of 0) 
func load_keybind_settings(id: int):
	print("loading keybindings for player %s" %[id])
	var keybind_settings = {}
	var events = ["jump", "sprint", "roll", "attack", "throw", "low_burn", 
				"pewter", "iron", "steel"]

	for event in events: 
		var key_name = null
		var backup = null
		
		for key in InputMap.action_get_events(event):
			#print("E: %s\tK: %s\tI: %s" %[event, key, key.device])
			if key.device == id:
				key_name = key.as_text()
				#if key is InputEventJoypadButton:
					#var button_name = InputEventJoypadButton.get_joy_button_string(key.button_index)
					#print("Generic button name: %s" % button_name)

			elif key.device == -1: # Defaults to all devices
				backup = key.as_text()

		if key_name != null:
			keybind_settings[event] = key_name
		elif backup != null:
			keybind_settings[event] = backup 
		else:
			keybind_settings[event] = "Unbound"
					
	return keybind_settings

## Called when a button from the Player Group is pressed. Sets the keybind 
## buttons to the proper name for the player button pressed. 
func on_button_press(button: BaseButton):
	print("button pressed %s %s" %[button.name, button.name.to_int()])
	# The buttons are name 1-8 but the id's go 0-7 
	load_keybinds_for_device(button.name.to_int() - 1) 
	print(Input.get_joy_name(0))
	
 
## Finds the keybind currently mapped to an action for a specified device. 
func get_action_event_keybound(event: String, id: int) -> InputEvent:
	for key in InputMap.action_get_events(event):
		#print("DEVICE %s\tID %s" %[key.device, id])
		if key.device == id:
			#print("YES -> E: %s\tK: %s\tI: %s" %[event, key, key.device])
			return key
		#else:
			#print("NO -> E: %s\tK: %s\tI: %s" %[event, key, key.device])
	return null

## When the apply button is pressed, save the graphics setting.
func _on_apply_pressed() -> void:
	ConfigFileHandler.save_keybind_settings()
	
	
	
