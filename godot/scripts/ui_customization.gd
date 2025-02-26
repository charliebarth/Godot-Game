extends TabBar

@onready var grid_container: GridContainer = $MarginContainer/ScrollContainer/VBoxContainer/UI_Placement/VBoxContainer/PanelContainer/GridContainer

@onready var size_label: Label = $MarginContainer/ScrollContainer/VBoxContainer/UI_Size_Opacity/HBoxContainer/VBoxContainer/HBoxContainer/Value
@onready var opacity_label: Label = $MarginContainer/ScrollContainer/VBoxContainer/UI_Size_Opacity/HBoxContainer/VBoxContainer/HBoxContainer2/Value

@onready var player_ui: Control = $MarginContainer/ScrollContainer/VBoxContainer/UI_Size_Opacity/HBoxContainer/PlayerUI

@onready var size_slider: HSlider = $MarginContainer/ScrollContainer/VBoxContainer/UI_Size_Opacity/HBoxContainer/VBoxContainer/HBoxContainer/HSlider
@onready var opacity_slider: HSlider = $MarginContainer/ScrollContainer/VBoxContainer/UI_Size_Opacity/HBoxContainer/VBoxContainer/HBoxContainer2/HSlider

var button_group = ButtonGroup.new()

var ui_size = null; 
var ui_opacity = null;
var pos = null;

func _ready():
	# Create the button group for the ui placement 
	for button in grid_container.get_children():
		if button is Button:
			button.button_group = button_group
	
	button_group.connect("pressed", on_button_press)
	
	# Load in the UI settings and set 
	var UI_settings = ConfigFileHandler.load_settings_helper("ui")
	print(UI_settings["size"], UI_settings["opacity"])
	
	scale_UI_size(UI_settings["size"])
	size_slider.value = UI_settings["size"]
	
	scale_UI_opacity(UI_settings["opacity"])
	opacity_slider.value = UI_settings["opacity"]
	
	pos = UI_settings["pos"]
	button_group.get_buttons()[0 if pos == null else pos].button_pressed = true
	

func scale_UI_size(val: float) -> void:
	size_label.text = str(val);
	# scale the UI elements to show up the same as they will in game
	player_ui.scale = Vector2(2 * val + 1, 2 * val + 1);
	ui_size = val;
	
func scale_UI_opacity(val: float) -> void:
	opacity_label.text = str(val);
	player_ui.modulate.a = val;
	ui_opacity = val;
	

# When a UI placement button is pressed
func on_button_press(button: BaseButton):
	print("button pressed %s" %[button.name])
	pos = button.name.to_int()


## When the apply button is pressed, save the ui setting.
func _on_apply_pressed() -> void:
	ConfigFileHandler.save_ui_settings(
		1 if ui_size == null else ui_size, 
		1 if ui_opacity == null else ui_opacity, 
		0 if pos == null else pos,
		);
