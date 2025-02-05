extends Node

var config = ConfigFile.new()
const SETTINGS_FILE_PATH = "user://settings.ini"

## Called when the node enters the scene tree for the first time.
func _ready() -> void:
	if !FileAccess.file_exists(SETTINGS_FILE_PATH):
		config.set_value("audio", "Master", 0.6)
		config.set_value("audio", "Music", 0.6)
		config.set_value("audio", "SFX", 0.6)
		config.set_value("audio", "PlayerSFX", 0.6)
		config.set_value("audio", "WorldSFX", 0.6)
		
		config.set_value("graphics", "fullscreen", true)
		config.set_value("graphics", "borderless", false)
		config.set_value("graphics", "vsync", false)
	
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
		"borderless", 
		DisplayServer.window_get_flag(DisplayServer.WINDOW_FLAG_BORDERLESS)
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
	
