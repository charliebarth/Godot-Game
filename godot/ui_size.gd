extends Control

@onready var size_label: Label = $HBoxContainer/VBoxContainer/HBoxContainer/Value
@onready var opacity_label: Label = $HBoxContainer/VBoxContainer/HBoxContainer2/Value

@onready var player_ui: Control = $HBoxContainer/PlayerUI

@onready var size_slider: HSlider = $HBoxContainer/VBoxContainer/HBoxContainer/HSlider
@onready var opacity_slider: HSlider = $HBoxContainer/VBoxContainer/HBoxContainer2/HSlider


var ui_size = null; 
var ui_opacity = null;

func _ready() -> void:
	var UI_settings = ConfigFileHandler.load_ui_settings()
	print(UI_settings["size"], UI_settings["opacity"])
	
	scale_UI_size(UI_settings["size"])
	size_slider.value = UI_settings["size"]
	
	scale_UI_opacity(UI_settings["opacity"])
	opacity_slider.value = UI_settings["opacity"]

func scale_UI_size(val: float) -> void:
	size_label.text = str(val);
	player_ui.scale = Vector2(val, val);
	ui_size = val;
	
func scale_UI_opacity(val: float) -> void:
	opacity_label.text = str(val);
	player_ui.modulate.a = val;
	ui_opacity = val;

## When the apply button is pressed, save the graphics setting.
func _on_apply_pressed() -> void:
	ConfigFileHandler.save_ui_settings(
		1 if ui_size == null else ui_size, 
		1 if ui_opacity == null else ui_opacity);
