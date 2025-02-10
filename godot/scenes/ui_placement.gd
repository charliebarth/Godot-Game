extends Control

var button_group = ButtonGroup.new()

var positions = []

func _ready():
	for button in $VBoxContainer/PanelContainer/GridContainer.get_children():
		if button is Button:
			button.button_group = button_group
	
	button_group.connect("pressed", on_button_press)
	
	

func on_button_press(button: BaseButton):
	print("button pressed %s" %[button.name])
	
	
	
