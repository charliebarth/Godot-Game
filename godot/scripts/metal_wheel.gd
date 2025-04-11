extends CanvasLayer
@onready var player = self.get_parent().get_parent() as Player
@onready var metal_reserve_bar_manager: MetalReserveBarManager = player.get_node("PlayerUI/MetalReserveBarManager")

# The name of the metal we are removing 
var prev_metal = null
# The event mapped to the metal we are removing
var rebind_event = null
# Types of metals
var metals = ["iron", "steel", "pewter", "copper", "bronze", "tin", 
			"duralumin", "nicrosil", "chromium", "gold"]

func _process(delta: float) -> void:
	# When the button to open the metal selector is pressed
	if Input.is_action_just_pressed("metal_selector"):
		$Label.visible = false # Make sure the label is hiddle
		$SelectionWheel.show() # Show the selector wheel 
		
	# When the button is released 
	elif Input.is_action_just_released("metal_selector"):
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


func display_msg(msg: String) -> void:
	$Label.text = msg
	$Label.visible = true
	await get_tree().create_timer(3).timeout 
	$Label.visible = false


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


func get_action_from_event(event: InputEvent):
	for action in metals:
		if InputMap.event_is_action(event, action, true):
			rebind_event = event
			return action
	return "Not a metal"
