extends TabBar

@onready var display_mode_btn = $MarginContainer/VBoxContainer/DisplayMode/HBoxContainer/DisplayModeBtn
@onready var border_mode_btn = $MarginContainer/VBoxContainer/BorderMode/HBoxContainer/BorderModeBtn
@onready var vsync_mode_btn = $MarginContainer/VBoxContainer/VsyncMode/HBoxContainer/VsyncModeBtn
@onready var window_size_btn = $MarginContainer/VBoxContainer/WindowSize/HBoxContainer/WindowSizeBtn
@onready var fps_slider = $"MarginContainer/VBoxContainer/FPS cap/HBoxContainer/HSlider"
@onready var fps_label = $"MarginContainer/VBoxContainer/FPS cap/HBoxContainer/Value"

var resolutions = [Vector2i(1920, 1080), Vector2i(1920,1200), 
						Vector2i(1080,960), Vector2i(1280, 720), Vector2i(800,600)]
						
func _ready():
	var graphics_settings = ConfigFileHandler.load_settings_helper("graphics")
	
	set_window_mode(1 if graphics_settings["fullscreen"] else 0)
	display_mode_btn.selected = graphics_settings["fullscreen"]
	
	set_window_size(graphics_settings["size"])
	window_size_btn.selected = graphics_settings["size"]
	
	set_border_mode(graphics_settings["borderless"])
	border_mode_btn.button_pressed = graphics_settings["borderless"]
	
	set_vsync(graphics_settings["vsync"])
	vsync_mode_btn.button_pressed = graphics_settings["vsync"]
	
	set_fps(graphics_settings["fps"])
	fps_slider.value = graphics_settings["fps"]
	
	print("\nloading window mode to %s" %[DisplayServer.window_get_mode()])
	print("loading borderless %s" %[DisplayServer.window_get_flag(DisplayServer.WINDOW_FLAG_BORDERLESS)])
	print("loading vsync %s" %[DisplayServer.window_get_vsync_mode()])
	print("loading window size %s\n" %[DisplayServer.window_get_size()])

func set_window_mode(index: int) -> void:
	DisplayServer.window_set_mode(
		DisplayServer.WINDOW_MODE_WINDOWED if index == 0 
		else DisplayServer.WINDOW_MODE_FULLSCREEN)
	print("set window mode to %s" %[DisplayServer.window_get_mode()])

func set_border_mode(enabled: bool) -> void:
	DisplayServer.window_set_flag(DisplayServer.WINDOW_FLAG_BORDERLESS,enabled)
	print("set borderless to %s" %[DisplayServer.window_get_flag(DisplayServer.WINDOW_FLAG_BORDERLESS)])

func set_vsync(enabled: bool):
	DisplayServer.window_set_vsync_mode(
		DisplayServer.VSYNC_ENABLED if enabled 
		else DisplayServer.VSYNC_DISABLED)
	print("set vsync to %s" %[DisplayServer.window_get_vsync_mode()])

func set_window_size(index: int) -> void:
	DisplayServer.window_set_size(resolutions[index])
	print("set window size to %s" %[DisplayServer.window_get_size()])

func set_fps(val: float) -> void:
	Engine.max_fps = val
	fps_label.text = str(val)
	print("set max fps to %s" %[Engine.max_fps])
	
## When the apply button is pressed, save the graphics setting.
func _on_apply_pressed() -> void:
	ConfigFileHandler.save_graphics_setting()
