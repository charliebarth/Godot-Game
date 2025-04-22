## Handles the logic for remapping metals with the metal wheel selector.
##
## @author Trinity Pittman 
## @version Spring 2025
extends CanvasLayer

## A reference to the player that has this metal wheel
@onready var player = self.get_parent().get_parent() as Player
## A reference to the players Metal Reserve Bar Manager
@onready var metal_reserve_bar_manager: MetalReserveBarManager = player.get_node("PlayerUI/MetalReserveBarManager")
## A reference to the players Input Manager
@onready var input_manager: InputManager = player.get_node("InputManager")

## The name of the metal we are removing 
var prev_metal = null
## The event mapped to the metal we are removing
var rebind_event = null
## Keeps track of whether the button was just pressed
var pressed = false
## Types of metals
var metals = ["iron", "steel", "pewter", "copper", "bronze", "tin", 
			"duralumin", "nicrosil", "chromium", "gold"]
## This represents the number of seconds the label will stay visible after a 
## message is displayed on screen
const SECONDS_BEFORE_INVISIBLE = 3

## This function is called every frame. It will check if the metal selector 
## wheel button is pressed, if it is it will show the wheel. If the button is 
## just released, it performs the logic to change the metal keybound. 
##
## @param `delta` (float) - The time in seconds since the last frame.
func _process(delta: float) -> void:
	# When the button to open the metal selector is pressed
	var metal_selector_event = input_manager.str_to_player_event("metal_selector")
	if input_manager.check_for_player_event(metal_selector_event):
		$Label.visible = false # Make sure the label is hiddle
		$SelectionWheel.show() # Show the selector wheel 
		pressed = true
	# When the button is released 
	elif !input_manager.check_for_player_event(metal_selector_event) and pressed:
		# Get the metal that was chosen, this will be the new metal
		var new_metal: String = $SelectionWheel.close().to_lower()
		
		# Check conditions for the metal to unbind
		if prev_metal == "Not a metal": # If the button pressed isn't a metal
			display_msg("The button pressed is not mapped to a metal")
			
		elif prev_metal == null: # If a button was not pressed 
			display_msg("Press the button you want to bind the metal to after opening the wheel")
			
		else: 
			# Check if we need to add the new metal or not
			if metal_reserve_bar_manager.add_remove(prev_metal, new_metal):
				# Change the keybindings 
				InputMap.action_erase_event(prev_metal, rebind_event)
				InputMap.action_add_event(new_metal, rebind_event)
				display_msg(prev_metal + " changed to " + new_metal)
				
			else: # If the new metal was already on screen 
				display_msg(new_metal + " already on screen")
		prev_metal = null 
		pressed = false

## This is a helper function to display a message on screen to a player. The 
## visibility will be set to false after a specified number of seconds. 
## 
## @param `msg` (String) - The message to display to the screen 
func display_msg(msg: String) -> void:
	$Label.text = msg
	$Label.visible = true
	await get_tree().create_timer(SECONDS_BEFORE_INVISIBLE).timeout 
	$Label.visible = false

## This function is called when an input event is detected. It checks if the 
## selection wheel is visible, if it is we make sure the conditions are met for 
## the input event given to be used as a remappable button, if they are the 
## prev_metal variable is set.
## 
## @param `event` (InputEvent) - The input event that was detected
func _input(event: InputEvent) -> void:
	if $SelectionWheel.is_visible_in_tree() and prev_metal == null:
		# The event cannot be the metal selector button
		var is_relevant_joy_button = event is InputEventJoypadButton and event.button_index != 11
		# The event cannot be the movement buttons
		var is_relevant_joy_motion = event is InputEventJoypadMotion and event.axis not in [0,1,2,3]
		# The event has to be from the same device as the player 
		var is_this_player = player.get_device_id() == event.device
		
		if (is_relevant_joy_button or is_relevant_joy_motion) and is_this_player:
			# Find the action mapped to this event 
			prev_metal = get_action_from_event(event)

## This function given an event, returns the name of the action bound to that 
## event if it is a metal, otherwise it returns "Not a metal". 
##
## @param `event` (InputEvent) - The input event to get the action name of
func get_action_from_event(event: InputEvent):
	for action in metals:
		if InputMap.event_is_action(event, action, true):
			rebind_event = event
			return action
	return "Not a metal"
