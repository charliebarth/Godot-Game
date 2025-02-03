extends Control

@onready var display_mode_btn = $HBoxContainer/DisplayModeBtn

func ready():
	pass
	


func _on_display_mode_btn_item_selected(index: int) -> void:
	var mode = null
	if index == 0:
		mode = DisplayServer.WINDOW_MODE_FULLSCREEN 
	else: 
		mode = DisplayServer.WINDOW_MODE_WINDOWED
	DisplayServer.window_set_mode(mode)


func _on_check_button_toggled(toggled_on: bool) -> void:
	DisplayServer.window_set_flag(DisplayServer.WINDOW_FLAG_BORDERLESS,toggled_on)
