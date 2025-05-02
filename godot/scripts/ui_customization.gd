extends TabBar
## Handles the logic for the UI customization settings tab 
## 
## @author Trinity Pittman
## @version Spring 2025

## Container that contains the UI Placement buttons
@onready var grid_container: GridContainer = $MarginContainer/ScrollContainer/VBoxContainer/UI_Placement/VBoxContainer/PanelContainer/GridContainer

## The label representing the current size of the UI
@onready var size_label: Label = $MarginContainer/ScrollContainer/VBoxContainer/UI_Size_Opacity/HBoxContainer/VBoxContainer/HBoxContainer/Value
## The label representing the current opacity of the UI
@onready var opacity_label: Label = $MarginContainer/ScrollContainer/VBoxContainer/UI_Size_Opacity/HBoxContainer/VBoxContainer/HBoxContainer2/Value

## A representation of how the UI elements will display in game 
@onready var player_ui: Control = $MarginContainer/ScrollContainer/VBoxContainer/UI_Size_Opacity/HBoxContainer/PlayerUI

## The slider that changes the current size of the UI
@onready var size_slider: HSlider = $MarginContainer/ScrollContainer/VBoxContainer/UI_Size_Opacity/HBoxContainer/VBoxContainer/HBoxContainer/HSlider
## The slider that changes the current opacity of the UI
@onready var opacity_slider: HSlider = $MarginContainer/ScrollContainer/VBoxContainer/UI_Size_Opacity/HBoxContainer/VBoxContainer/HBoxContainer2/HSlider

## Button group containing the UI placement buttons
var button_group = ButtonGroup.new()


var ui_size = null; ## Holds the current UI size
var ui_opacity = null; ## Holds the current UI opacity
var pos = null; ## Holds the current position of the UI


## Called when this node enters the scene tree for the first time. Adds the UI
## placement buttons to a button group, and loads the settings that were saved
## in the config file. 
func _ready() -> void:
	# Create the button group for the ui placement 
	for button in grid_container.get_children():
		if button is Button:
			button.button_group = button_group
	
	button_group.connect("pressed", on_button_press)
	
	# Load in the UI settings and set 
	var UI_settings = ConfigFileHandler.load_settings_helper("ui")
	
	size_slider.value = UI_settings["size"]
	scale_UI_size(UI_settings["size"])
	
	
	opacity_slider.value = UI_settings["opacity"]
	scale_UI_opacity(UI_settings["opacity"])
	
	pos = UI_settings["pos"]
	button_group.get_buttons()[0 if pos == null else pos].button_pressed = true


## Called when the UI size slider is moved. This function updates the player ui
## representation and sets the ui_size variable. 
##
## @param val The value the UI size slider was moved to 
func scale_UI_size(val: float) -> void:
	size_label.text = str(val);
	# scale the UI elements to show up the same as they will in game
	player_ui.scale = Vector2(2 * val + 1, 2 * val + 1);
	player_ui.queue_redraw()
	ui_size = val;

## Called when the UI scale slider is moved. This function updates the player ui
## representation and sets the ui_opacity variable. 
## 
## @param val The value the UI opacity slider was moved to 
func scale_UI_opacity(val: float) -> void:
	opacity_label.text = str(val);
	player_ui.modulate.a = val;
	ui_opacity = val;
	

## Called when a UI placement button is pressed. Sets the pos variable based on 
## the button pressed. 
##
## @param button The button that was pressed 
func on_button_press(button: BaseButton) -> void:
	pos = button.name.to_int()


## When the apply button is pressed, save the ui settings to the config file.
func _on_apply_pressed() -> void:
	ConfigFileHandler.save_ui_settings(
		1 if ui_size == null else ui_size,
		1 if ui_opacity == null else ui_opacity,
		0 if pos == null else pos,
		);
