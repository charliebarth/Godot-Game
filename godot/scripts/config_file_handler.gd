extends Node

var config = ConfigFile.new()
const SETTINGS_FILE_PATH = "user://settings.ini"

var resolutions = [Vector2i(1920, 1080), Vector2i(1920,1200), 
						Vector2i(1080,960), Vector2i(1280, 720), Vector2i(800,600)]

## Called when the node enters the scene tree for the first time.
func _ready() -> void:
	if !FileAccess.file_exists(SETTINGS_FILE_PATH):
		config.set_value("audio", "Master", 0.6)
		config.set_value("audio", "Music", 0.6)
		config.set_value("audio", "SFX", 0.6)
		config.set_value("audio", "PlayerSFX", 0.6)
		config.set_value("audio", "WorldSFX", 0.6)
		
		config.set_value("graphics", "fullscreen", true)
		config.set_value("graphics", "size", 0)
		config.set_value("graphics", "borderless", false)
		config.set_value("graphics", "vsync", false)
		config.set_value("graphics", "fps", 60)
	
		config.save(SETTINGS_FILE_PATH)
	else:
		config.load(SETTINGS_FILE_PATH)

## Save the audio setting to the config file.
func save_audio_setting(key: String, value: float) -> void:
	config.set_value("audio", key, value)
	config.save(SETTINGS_FILE_PATH)

## Load the audio settings from the config file.
func load_audio_settings():
	var audio_settings = {}
	
	for key in config.get_section_keys("audio"):
		audio_settings[key] = config.get_value("audio", key)
	
	return audio_settings

func save_graphics_setting():
	config.set_value(
		"graphics", 
		"fullscreen", 
		DisplayServer.window_get_mode() == DisplayServer.WINDOW_MODE_FULLSCREEN
	)
	config.set_value(
		"graphics",
		"size",
		resolutions.find(DisplayServer.window_get_size())
	)
	config.set_value(
		"graphics", 
		"borderless", 
		DisplayServer.window_get_flag(DisplayServer.WINDOW_FLAG_BORDERLESS)
	)
	config.set_value(
		"graphics",
		"fps",
		Engine.max_fps
	)
	config.set_value(
		"graphics", 
		"vsync", 
		DisplayServer.window_get_vsync_mode() == DisplayServer.VSYNC_ENABLED
	)

	config.save(SETTINGS_FILE_PATH)
	print("saved graphics settings")
		
func load_graphics_settings():
	var graphics_settings = {}
	
	for key in config.get_section_keys("graphics"):
		graphics_settings[key] = config.get_value("graphics", key)
	
	return graphics_settings
	
func save_ui_settings(size: float, opacity: float):
	config.set_value(
		"ui",
		"size",
		size
	)
	config.set_value(
		"ui",
		"opacity",
		opacity
	)
	
	config.save(SETTINGS_FILE_PATH)
	print("saved ui settings")

func load_ui_settings():
	var ui_settings = {}
	for key in config.get_section_keys("ui"):
		ui_settings[key] = config.get_value("ui", key)
		
	return ui_settings

func save_keybind_settings():
	var events = ["jump", "roll", "sprint", "iron", "pewter"]
	for i in 8:
		for event in events:
			print(event + "   "+ InputMap.action_get_events(event)[0].as_text())
			#config.set_value(
				#"keybinds",
				#event,
				#InputMap.action_get_events(event).as_text()
			#)

func load_keybind_settings(id: int):
	print("loading keybindings for player %s" %[id])
	var keybind_settings = {}
	var events = ["jump", "sprint", "roll", "attack", "throw", "low_burn", "pewter", "iron", "steel"]

	for event in events: 
		for key in InputMap.action_get_events(event):
			#print("E: %s\tK: %s\tI: %s" %[event, key, key.device])
			if key.device == id or key.device == -1:
				keybind_settings[event] = key.as_text()

	return keybind_settings

	
