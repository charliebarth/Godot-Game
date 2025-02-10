extends Control

@onready var size_label: Label = $VBoxContainer/HBoxContainer/Value


func scale_UI_size(val: float) -> void:
	size_label.text = str(val);
