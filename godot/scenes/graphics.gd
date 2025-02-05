extends TabBar

@onready var display_mode_btn = $MarginContainer/VBoxContainer/DisplayMode/HBoxContainer/DisplayModeBtn
@onready var border_mode_btn = $MarginContainer/VBoxContainer/BorderMode/HBoxContainer/BorderModeBtn
@onready var vsync_mode_btn = $MarginContainer/VBoxContainer/VsyncMode/HBoxContainer/VsyncModeBtn

func _ready():
	var graphics_settings = ConfigFileHandler.load_graphics_settings()
	
	set_window_mode(1 if graphics_settings["fullscreen"] else 0)
	display_mode_btn.selected = graphics_settings["fullscreen"]
	
	set_border_mode(graphics_settings["borderless"])
	border_mode_btn.button_pressed = graphics_settings["borderless"]
	
	set_vsync(graphics_settings["vsync"])
	vsync_mode_btn.button_pressed = graphics_settings["vsync"]
	
	print("loading window mode to %s" %[DisplayServer.window_get_mode()])
	print("loading borderless %s" %[DisplayServer.window_get_flag(DisplayServer.WINDOW_FLAG_BORDERLESS)])

func set_window_mode(index: int) -> void:
	DisplayServer.window_set_mode(
		DisplayServer.WINDOW_MODE_WINDOWED if index == 0 
		else DisplayServer.WINDOW_MODE_FULLSCREEN)
	print("set window mode to %s" %[DisplayServer.window_get_mode()])

func set_border_mode(enabled: bool) -> void:
	DisplayServer.window_set_flag(DisplayServer.WINDOW_FLAG_BORDERLESS,enabled)
	print("borderless %s" %[DisplayServer.window_get_flag(DisplayServer.WINDOW_FLAG_BORDERLESS)])

func set_vsync(enabled: bool):
	DisplayServer.window_set_vsync_mode(
		DisplayServer.VSYNC_ENABLED if enabled 
		else DisplayServer.VSYNC_DISABLED)
		
## When the apply button is pressed, save the graphics setting.
func _on_apply_pressed() -> void:
	ConfigFileHandler.save_graphics_setting()
