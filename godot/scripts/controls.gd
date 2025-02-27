extends TabBar
## Handles the logic for setting the keybind settings menu and rebinding keys.
## 
## @author Trinity Pittman
## @version Spring 2025
class_name Controls

## The Hbox that contains the Player # buttons
@onready var h_box_container: HBoxContainer = $"MarginContainer/ScrollContainer/VBoxContainer/Player Choice/HBoxContainer"
## The Vbox that contains the rebind control nodes
@onready var v_box_container: VBoxContainer = $MarginContainer/ScrollContainer/VBoxContainer

## Types of actions the keybind menu controls
var actions = ["jump", "sprint", "roll", "attack", "throw", "low_burn", 
				"pewter", "iron", "steel"]
## The button group that holds the Player # buttons
var button_group = ButtonGroup.new()


## Called when this node is added to the scene tree. Sets up the player group 
## and loads the keybindings from the save file. 
func _ready() -> void:
	# Set up Player Button Group 
	for button in h_box_container.get_children():
		if button is Button:
			button.button_group = button_group
	
	button_group.connect("pressed", on_button_press)	
	
	set_keybinds_from_config()
	
	# Load Keybinds for Player 1 on startup 
	set_device_keybind_menu(0)


## Loads the keybindings from the config file. 
func set_keybinds_from_config() -> void:
	var keybind_settings = ConfigFileHandler.load_all_keybind_settings()

	# Erase current bindings
	for action in actions:
		InputMap.action_erase_events(action)
		
	# Add keybinding for each player 
	for player in keybind_settings: 
		for action in player: 
			InputMap.action_add_event(action, player[action])


## Sets the keybind menu for a specific player 
##
## @param id The player id to set the keybind menu for (Player 1 id is 0)
func set_device_keybind_menu(id: int) -> void:
	var keybind_settings = load_keybind_settings(id)
	
	for el in v_box_container.get_children():
		if el.has_method("set_button_text"):
			var latest_event = get_action_event_keybound(
				el.action_name, 
				button_group.get_pressed_button().name.to_int() - 1)
			el.set_button_text(keybind_settings, latest_event)


## Gets the keybind settings for a player based on id 
## 
## @param id Represents the id of a player (Player 1 has an id of 0) 
## @returns A dictionary of the keybind settings 
func load_keybind_settings(id: int) -> Dictionary:
	var keybind_settings = {}

	# Go through the actions 
	for action in actions: 
		var key_name = null
		var backup = null
		# Get the events mapped to the action
		for key in InputMap.action_get_events(action):
			# Try to find the event for this device id
			if key.device == id: 
				key_name = key.as_text()
			elif key.device == -1: # Defaults to all devices
				backup = key.as_text()

		if key_name != null: # If we found the right key for the specific device
			keybind_settings[action] = key_name
		elif backup != null: # The general key that all devices use 
			keybind_settings[action] = backup 
		else:
			keybind_settings[action] = "Unbound"
					
	return keybind_settings


## Called when a button from the Player Group is pressed. Sets the keybind 
## buttons to the proper name for the player button pressed. 
## 
## @param button The button that was pressed 
func on_button_press(button: BaseButton) -> void:
	print("button pressed %s %s" %[button.name, button.name.to_int()])
	# The buttons are name 1-8 but the id's go 0-7 
	set_device_keybind_menu(button.name.to_int() - 1) 
	print(Input.get_joy_name(0))

 
## Finds the keybind currently mapped to an action for a specified device. 
## 
## @param action Gets the current event mapped to the specified action
## @returns The action event that is mapped, or null 
func get_action_event_keybound(action: String, id: int) -> InputEvent:
	for key in InputMap.action_get_events(action):
		if key.device == id:
			return key
	return null


## When the apply button is pressed, save the graphics setting.
func _on_apply_pressed() -> void:
	ConfigFileHandler.save_keybind_settings()
