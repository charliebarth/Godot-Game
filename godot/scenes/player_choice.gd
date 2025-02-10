extends Control

var button_group = ButtonGroup.new()

func _ready():
	for button in $HBoxContainer.get_children():
		if button is Button:
			button.button_group = button_group
	
	button_group.connect("pressed", on_button_press)
	
	

func on_button_press(button: BaseButton):
	print("button pressed %s" %[button.name])
	var keybindings = ConfigFileHandler.load_keybind_settings(button.name.hash())
	
	
	
