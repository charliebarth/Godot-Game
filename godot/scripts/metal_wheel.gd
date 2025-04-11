extends CanvasLayer
@onready var player = self.get_parent().get_parent() as Player
@onready var metal_reserve_bar_manager: MetalReserveBarManager = player.get_node("PlayerUI/MetalReserveBarManager")

var to_rebind = null
var rebind_event = null

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("metal_selector"):
		$SelectionWheel.show()
	elif Input.is_action_just_released("metal_selector"):
		var metal = $SelectionWheel.close()
		print("SELECTED: ", metal)
		print("Try Rebind ", to_rebind, " to ", metal)
		if to_rebind != null:
			InputMap.action_erase_event(to_rebind, rebind_event)
			InputMap.action_add_event(metal, rebind_event)
			metal_reserve_bar_manager.add_remove(to_rebind, metal)
			print("Rebound ", to_rebind, " to ", metal)
			to_rebind = null
		
		
func _input(event: InputEvent) -> void:
	#input = InputEventJoypadButton.new()
	#input.button_index = json["JoypadButton"]
	if $SelectionWheel.is_visible_in_tree() and to_rebind == null:
		if ((event is InputEventJoypadButton and event.button_index != 11) or (event is InputEventJoypadMotion)) and (player.get_device_id() == event.device):
			print("--FINDING: %s DEVICE: %s" %[event, event.device])
			to_rebind = get_action_from_event(event)
			print("--FOUND: ", to_rebind)

func get_action_from_event(event: InputEvent):
	var metals = ["iron", "steel", "pewter", "copper", "bronze", "tin"]
	for action in InputMap.get_actions():
		if action in metals and !(action.contains("ui_")):
			print("ACTION: ", action)
			if InputMap.event_is_action(event, action):
				print("FOUND IT ", action)
				rebind_event = event
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
