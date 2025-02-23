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
	
	# Load Keybinds for Player 1 on startup 
	load_keybinds_for_device(0)
	
	
func load_keybinds_for_device(id: int):
	var keybind_settings = ConfigFileHandler.load_keybind_settings(id)
	print(keybind_settings, "\n")
	
	for el in v_box_container.get_children():
		if el.has_method("set_button_text"):
			var latest_event = get_action_event_keybound(el.action_name, 
														button_group
														.get_pressed_button()
														.name
														.to_int()-1)
			print("LATEST EV: ", latest_event)
			el.set_button_text(keybind_settings, latest_event)
	

## Called when a button from the Player Group is pressed. Sets the keybind 
## buttons to the proper name for the player button pressed. 
func on_button_press(button: BaseButton):
	print("button pressed %s %s" %[button.name, button.name.to_int()])
	# The buttons are name 1-8 but the id's go 0-7 
	load_keybinds_for_device(button.name.to_int() - 1) 
	
 
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

	
	
	
