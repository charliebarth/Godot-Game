## Handles the logic for rebinding a button.
## 
## @author Trinity Pittman
## @version Spring 2025
extends Control

## The label that contains what action the rebind button controls
@onready var label: Label = $HBoxContainer/Label as Label
## The button to rebind a key, shows what key an action is bound to 
@onready var button: Button = $HBoxContainer/Button as Button

## The action name (jump, sprint, etc.)
@export var action_name: String = "Unassigned"
## The last event that was bound to this action
var latest_event: InputEvent = null


## Called when the node enters the scene. Sets the label depending on the action
## name. Also turns of unhandled input processing. 
func _ready() -> void:
	set_process_unhandled_input(false)
	set_action_label()


## Sets the action label based on the action name. 
func set_action_label() -> void:
	label.text = "Unassigned"

	# Match the names in godot settings to the names we want to display
	match action_name:
		"jump":
			label.text = "Jump"
		"sprint":
			label.text = "Sprint"
		"roll":
			label.text = "Roll"
		"attack":
			label.text = "Melee Attack"
		"throw":
			label.text = "Throw"
		"low_burn":
			label.text = "Low Burn"
		"pewter":
			label.text = "Metal 1"
		"iron":
			label.text = "Metal 2"
		"steel":
			label.text = "Metal 3"


## Sets the button text given the keybind settings, also recieves the latest 
## event and stores this. 
## 
## @param `keybind_settings` (Dictionary) - contains all of the keybinds for a 
## 											single player
## @param `event` (InputEvent) - The last event that was keybound to the action 
func set_button_text(keybind_settings: Dictionary, event: InputEvent) -> void:
	button.text = controller_matcher(keybind_settings[action_name])
	latest_event = event


## Matches an InputEvent to its Xbox key name
##
## @param `event` (InputEvent) - the input event to map to a key name
## @returns - The string name of the input event 
func controller_matcher(event: InputEvent) -> String:
	var name = event.as_text()
	
	if event is InputEventJoypadMotion:
			match event.axis:
				4: name = "LT"
				5: name = "RT"
	elif event is InputEventJoypadButton:
			match event.button_index:
				0: name = "A"
				1: name = "B"
				2: name = "X"
				3: name = "Y"
				4: name = "Back"
				5: name = "Home"
				6: name = "Menu"
				7: name = "LS"
				9: name = "LB"
				10: name = "RB"
				11: name = "Up"
				12: name = "Down"
				13: name = "Left"
				14: name = "Right"
				15: name = "Share"
	return name


## Called when a rebind button is pressed. This function will set the text for 
## the button to prompt the user to press a new key. It will then turn on 
## processing of unhandled input and make all other keybind buttons 
## untoggleable.
## 
## @param `toggled_on` (bool) - Whether the button was toggled on or off
func _on_button_toggled(toggled_on: bool) -> void:
	if toggled_on:
		button.text = "Press a new key"
		# Catches input other nodes didn't handle
		set_process_unhandled_input(true)
		
	for i in get_tree().get_nodes_in_group("keybind_btns"):
		if i.action_name != self.action_name:
			i.button.toggle_mode = !toggled_on
			i.set_process_unhandled_input(false)


## When we press a key it will untoggle the button and call rebind
## 
## @param `event` (InputEvent) - The unhandled input event that was caught
func _unhandled_input(event: InputEvent) -> void:
	if ((event is InputEventJoypadButton or
		event is InputEventJoypadMotion or
		event is InputEventKey) and
		latest_event.device == event.device):
		# Makes it so you cannot rebind the sticks 
		if !(event is InputEventJoypadMotion and (event.axis in [0, 1, 2, 3])):
			rebind_action_key(event)
			button.button_pressed = false


## Rebinds an action to a new key. 
## 
## @param `event` (InputEvent) - The event to remap the action to.
func rebind_action_key(event: InputEvent) -> void:
	# Remove the event currently bound to the action if one exists
	if latest_event != null:
		InputMap.action_erase_event(action_name, latest_event)
	
	# Add the event and stop processing input
	InputMap.action_add_event(action_name, event)
	set_process_unhandled_input(false)
	button.text = controller_matcher(event)
	
	# Check to make sure no other actions have the same event 
	for rebind in get_tree().get_nodes_in_group("keybind_btns"):
		# If one does, unbind it 
		if rebind.button.text == button.text and rebind.button != button:
			rebind.button.emit_signal("toggled", true)
