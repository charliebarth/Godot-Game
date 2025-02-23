extends Control

@onready var label: Label = $HBoxContainer/Label as Label
@onready var button: Button = $HBoxContainer/Button as Button

@export var action_name: String = "Unassigned"
var latest_event: InputEvent = null

func _ready():
	set_process_unhandled_input(false)
	set_action_label()
	
	
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
			label.text = "Pewter"
		"iron":
			label.text = "Iron"
		"steel":
			label.text = "Steel"
	

func set_button_text(keybind_settings, event: InputEvent) -> void:
	button.text = keybind_settings[action_name]
	latest_event = event


func _on_button_toggled(toggled_on: bool) -> void:
	if toggled_on:
		button.text = "Press a new key"
		set_process_unhandled_input(true)
		
	for i in get_tree().get_nodes_in_group("keybind_btns"):
		if i.action_name != self.action_name:
			i.button.toggle_mode = !toggled_on
			i.set_process_unhandled_input(false)


## When we press a key it will untoggle the button
func _unhandled_input(event: InputEvent) -> void:
	print("UNHANDLED: %s DEVICE: %s" %[event, event.device])
	if ((event is InputEventJoypadButton or 
		event is InputEventJoypadMotion or 
		event is InputEventKey) and 
		latest_event.device == event.device): 
		rebind_action_key(event)
		button.button_pressed = false


func rebind_action_key(event) -> void:
	print("REBIND TO: ", event)
	if latest_event != null: 
		InputMap.action_erase_event(action_name, latest_event)
	InputMap.action_add_event(action_name, event)
	set_process_unhandled_input(false) 
	button.text = event.as_text()
	## set action name 
