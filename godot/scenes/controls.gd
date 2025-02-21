extends TabBar

@onready var jump_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Jump/HBoxContainer/jump
@onready var sprint_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Sprint/HBoxContainer/Button
@onready var roll_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Roll/HBoxContainer/Button

@onready var attack_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Melee/HBoxContainer/Button
@onready var throw_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Throw/HBoxContainer/Button

@onready var flare_btn: Button = $MarginContainer/ScrollContainer/VBoxContainer/Flare/HBoxContainer/Button
@onready var lowBurn_btn: Button = $"MarginContainer/ScrollContainer/VBoxContainer/Low Burn/HBoxContainer/Button"

@onready var metal1_btn: Button = $"MarginContainer/ScrollContainer/VBoxContainer/Metal 1/HBoxContainer/Button"
@onready var metal2_btn: Button = $"MarginContainer/ScrollContainer/VBoxContainer/Metal 2/HBoxContainer/Button"
@onready var metal3_btn: Button = $"MarginContainer/ScrollContainer/VBoxContainer/Metal 3/HBoxContainer/Button"
@onready var metal4_btn: Button = $"MarginContainer/ScrollContainer/VBoxContainer/Metal 4/HBoxContainer/Button"

@onready var h_box_container: HBoxContainer = $"MarginContainer/ScrollContainer/VBoxContainer/Player Choice/HBoxContainer"

var waiting = false
var action = ""
var button_group = ButtonGroup.new()
var keybind_btn_group = ButtonGroup.new()

func _ready():
	for button in h_box_container.get_children():
		if button is Button:
			button.button_group = button_group
	
	button_group.connect("pressed", on_button_press)	
	
	jump_btn.button_group = keybind_btn_group
	sprint_btn.button_group = keybind_btn_group
	roll_btn.button_group = keybind_btn_group
	attack_btn.button_group = keybind_btn_group
	throw_btn.button_group = keybind_btn_group
	flare_btn.button_group = keybind_btn_group
	lowBurn_btn.button_group = keybind_btn_group
	metal1_btn.button_group = keybind_btn_group
	metal2_btn.button_group = keybind_btn_group
	metal3_btn.button_group = keybind_btn_group
	metal4_btn.button_group = keybind_btn_group
	
	keybind_btn_group.connect("pressed", change_keybind)

func _input(event: InputEvent):
	if waiting:
		if event is InputEventJoypadButton:
			update_keybind(action, event)
			waiting = false
			

func on_button_press(button: BaseButton):
	print("button pressed %s" %[button.name])
	load_keybinds_for_device(button.name.hash())

func change_keybind(button: BaseButton):
	print("Changing keybindings")
	button.text = "Press a new key"
	action = button.name
	waiting = true
	
func update_keybind(action: String, event:InputEvent):
	var old_event = get_action_event_keybound(action, button_group
							.get_pressed_button()
							.name
							.hash())
	InputMap.action_erase_event(action, old_event)
	InputMap.action_add_event(action, event)
	print("Keybind", action, "changed to:", event, "from:", old_event)
	

func get_action_event_keybound(event: String, id: int) -> InputEvent:
	for key in InputMap.action_get_events(event):
		print("E: %s\tK: %s\tI: %s" %[event, key, key.device])
		if key.device == id:
			return key
	return null

func load_keybinds_for_device(id: int):
	var keybind_settings = ConfigFileHandler.load_keybind_settings(0)
	print(keybind_settings, "\n")
	var events = ["jump", "sprint", "roll", "attack", "throw", "low_burn", "pewter", "iron", "steel"]
	
	jump_btn.text = keybind_settings["jump"]
	sprint_btn.text = keybind_settings["sprint"]
	roll_btn.text = keybind_settings["roll"]
	
	attack_btn.text = keybind_settings["attack"]
	throw_btn.text = keybind_settings["throw"]
	#flare_btn.text = keybind_settings["flare"]
	lowBurn_btn.text = keybind_settings["low_burn"]
	
	# find what metals have keybinds and set these 
	metal1_btn.text = keybind_settings["pewter"] #TODO currently hardcoded ..
	metal2_btn.text = keybind_settings["iron"]
	metal3_btn.text = keybind_settings["steel"]
	#metal4_btn.text = keybind_settings["metal4"]
	
	
	
