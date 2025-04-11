extends CanvasLayer
@onready var player = self.get_parent().get_parent() as Player
@onready var metal_reserve_bar_manager: MetalReserveBarManager = 

var to_rebind = null

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("metal_selector"):
		$SelectionWheel.show()
		set_process_unhandled_input(true) 
	elif Input.is_action_just_released("metal_selector"):
		set_process_unhandled_input(false) 
		var metal = $SelectionWheel.close()
		print("SELECTED: ", metal)
		metal_reserve_bar_manager.add_remove(to_rebind, metal)
		print("Rebind ", to_rebind, " to ", metal)
		
		
func _unhandled_input(event: InputEvent) -> void:
	print("UNHANDLED: %s DEVICE: %s" %[event, event.device])
	
	
	if (event is InputEventJoypadButton) and (player.get_device_id() == event.device):
		to_rebind = get_action_from_event(event)
		print("Unhandled: ", to_rebind)

func get_action_from_event(event: InputEvent):
	for action in InputMap.get_actions():
		if InputMap.event_is_action(event, action):
			return action
	return null

#func get_event():
	#var key_name = null
	#var backup = null
	#var event = null
	## Get the next keybind input & see if its one of the metals 
	#for key in InputMap.action_get_events("Metal 1"):
		## Try to find the event for this device id
		#if key.device == player.get_device_id(): 
			#key_name = key
		#elif key.device == -1: # Defaults to all devices
			#backup = key
#
	#if key_name != null: # If we found the right key for the specific device
		#event = key_name
	#elif backup != null: # The general key that all devices use 
		#event = backup 
	#else:
		#event = "Unbound"
