extends GridContainer

var button_group = ButtonGroup.new()

func _ready():
	for button in .get_children():
		if button is Button:
			button.button_group = button_group
	
	button_group.connect("pressed", on_button_press)
	
	

func on_button_press(button: BaseButton):
	print("button pressed %s" %[button.name])
	## TODO Change UI configuration
	
	
