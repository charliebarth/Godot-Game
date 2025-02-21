extends TabBar

@onready var jump_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Jump/HBoxContainer/jump
@onready var sprint_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Sprint/HBoxContainer/sprint
@onready var roll_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Roll/HBoxContainer/roll

@onready var attack_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Melee/HBoxContainer/melee
@onready var throw_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Throw/HBoxContainer/throw

@onready var flare_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Flare/HBoxContainer/flare
@onready var lowBurn_btn: Button = $"MarginContainer/ScrollContainer/VBoxContainer/Low Burn/HBoxContainer/low_burn"

@onready var metal1_btn: Button = $"MarginContainer/ScrollContainer/VBoxContainer/Metal 1/HBoxContainer/Button"
@onready var metal2_btn: Button = $"MarginContainer/ScrollContainer/VBoxContainer/Metal 2/HBoxContainer/Button"
@onready var metal3_btn: Button = $"MarginContainer/ScrollContainer/VBoxContainer/Metal 3/HBoxContainer/Button"
@onready var metal4_btn: Button = $"MarginContainer/ScrollContainer/VBoxContainer/Metal 4/HBoxContainer/Button"

@onready var h_box_container: HBoxContainer = $"MarginContainer/ScrollContainer/VBoxContainer/Player Choice/HBoxContainer"
@onready var v_box_container: VBoxContainer = $MarginContainer/ScrollContainer/VBoxContainer

var waiting = false
var action = ""
var curr_btn = null
var button_group = ButtonGroup.new()
var keybind_btn_group = ButtonGroup.new()

func _ready():
	# Set up Player Button Group 
	for button in h_box_container.get_children():
		if button is Button:
			button.button_group = button_group
	
	button_group.connect("pressed", on_button_press)	
	
	# Set up Keybind Button Group 
	jump_btn.button_group = keybind_btn_group
	sprint_btn.button_group = keybind_btn_group
	roll_btn.button_group = keybind_btn_group
	attack_btn.button_group = keybind_btn_group
	throw_btn.button_group = keybind_btn_group
	#flare_btn.button_group = keybind_btn_group
	lowBurn_btn.button_group = keybind_btn_group
	metal1_btn.button_group = keybind_btn_group
	metal2_btn.button_group = keybind_btn_group
	metal3_btn.button_group = keybind_btn_group
	#metal4_btn.button_group = keybind_btn_group
	
	keybind_btn_group.connect("pressed", change_keybind)
	
	# Load Keybinds for Player 1 on startup 
	load_keybinds_for_device(0)

## Called when any input is recieved, this function will check for InputEvents 
## when 'waiting' and if one is found that is from the controller or keyboard 
## the event is sent to update the keybind. 
func _input(event: InputEvent):
	if waiting: # If we are currently waiting to rebind a key
		if (event is InputEventJoypadButton 
			or event is InputEventJoypadMotion 
			or event is InputEventKey): 
			update_keybind(action, event)
			waiting = false
			

## Called when a button from the Player Group is pressed. Sets the keybind 
## buttons to the proper name for the player button pressed. 
func on_button_press(button: BaseButton):
	print("button pressed %s %s" %[button.name, button.name.to_int()])
	# The buttons are name 1-8 but the id's go 0-7 
	load_keybinds_for_device(button.name.to_int() - 1) 
	
## Called when a button from the Keybind Group is pressed. Begins the process of
## changing a keybind for an action. 
func change_keybind(button: BaseButton):
	print("Changing keybindings of %s" %[button.name])
	button.text = "Press a new key"
	#TODO make it so other buttons cannot toggle 
	action = button.name # The action to change the keybind of (i.e. jump)
	waiting = true # To tell the input method to listen for InputEvents
	curr_btn = button 
	
## Changes a keybinding by first erasing the old one if it exists, then adding 
## the new event to the specified action. 
func update_keybind(action: String, event:InputEvent):
	var old_event = get_action_event_keybound(action, button_group.get_pressed_button().name.to_int())
							
	if old_event != null:
		InputMap.action_erase_event(action, old_event)
		
	InputMap.action_add_event(action, event)
	curr_btn.text = event.as_text()
	print("Keybind ", action, " changed to: ", event, " from: ", old_event)
	
## Finds the keybind currently mapped to an action for a specified device. 
func get_action_event_keybound(event: String, id: int) -> InputEvent:
	for key in InputMap.action_get_events(event):
		print("DEVICE %s\tID %s" %[key.device, id])
		if key.device == id:
			print("YES -> E: %s\tK: %s\tI: %s" %[event, key, key.device])
			return key
		else:
			print("NO -> E: %s\tK: %s\tI: %s" %[event, key, key.device])
	return null

## Loads the keybind settings for a specific device by recieving them from the 
## ConfigFile and 
func load_keybinds_for_device(id: int):
	var keybind_settings = ConfigFileHandler.load_keybind_settings(id)
	print(keybind_settings, "\n")
	var events = ["jump", "sprint", "roll", "attack", "throw", "low_burn", "pewter", "iron", "steel"]
	var buttons = keybind_btn_group.get_buttons()
	for i in len(buttons):
		print("Btn name: %s\tEvent name: %s" %[buttons[i].name, events[i]])
		buttons[i].text = keybind_settings[events[i]]
	
	#jump_btn.text = keybind_settings["jump"]
	#sprint_btn.text = keybind_settings["sprint"]
	#roll_btn.text = keybind_settings["roll"]
	#
	#attack_btn.text = keybind_settings["attack"]
	#throw_btn.text = keybind_settings["throw"]
	##flare_btn.text = keybind_settings["flare"]
	#lowBurn_btn.text = keybind_settings["low_burn"]
	#
	## find what metals have keybinds and set these 
	#metal1_btn.text = keybind_settings["pewter"] #TODO currently hardcoded ..
	#metal2_btn.text = keybind_settings["iron"]
	#metal3_btn.text = keybind_settings["steel"]
	#metal4_btn.text = keybind_settings["metal4"]
	
	
	
