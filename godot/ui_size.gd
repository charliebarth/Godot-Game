extends Control

@onready var size_label: Label = $HBoxContainer/VBoxContainer/HBoxContainer/Value
@onready var player_ui: Control = $HBoxContainer/PlayerUI
@onready var opacity_label: Label = $HBoxContainer/VBoxContainer/HBoxContainer2/Value

func scale_UI_size(val: float) -> void:
	size_label.text = str(val);
	player_ui.scale = Vector2(val, val);

func scale_UI_opacity(val: float) -> void:
	opacity_label.text = str(val);
	player_ui.modulate.a = val;
