extends Node

var config = ConfigFile.new()
const SETTINGS_FILE_PATH = "user://settings.ini"

var resolutions = [Vector2i(1920, 1080), Vector2i(1920,1200), 
				Vector2i(1080,960), Vector2i(1280, 720), Vector2i(800,600)]
var events = ["jump", "sprint", "roll", "attack", "throw", "low_burn", 
				"pewter", "iron", "steel"]

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
		
		config.set_value("ui", "size", 1)
		config.set_value("ui", "opacity", 1)
		config.set_value("ui", "pos_x", -475)
		config.set_value("ui", "pos_y", -270)
	
		config.save(SETTINGS_FILE_PATH)
	else:
		config.load(SETTINGS_FILE_PATH)

## Loads the setting based on the string passed in, for example "audio" or "ui".
func load_settings_helper(type: String):
	var settings = {}
	for key in config.get_section_keys(type):
		settings[key] = config.get_value(type, key)
	return settings

## Save the audio setting to the config file.
func save_audio_setting(key: String, value: float) -> void:
	config.set_value("audio", key, value)
	config.save(SETTINGS_FILE_PATH)

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

	
func save_ui_settings(size: float, opacity: float, pos_x, pos_y):
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
	config.set_value(
		"ui",
		"pos_x",
		pos_x
	)
	config.set_value(
		"ui",
		"pos_y",
		pos_y
	)
	
	config.save(SETTINGS_FILE_PATH)
	print("saved ui settings")

## UNUSED 
## 
func save_keybind_settings():
	for i in 8: # There are 8 players
		for event in events:	# Go through the list of events 
			for key in InputMap.action_get_events(event): # Thru bound keys 
				if key.device == i:		# Find devices bound key
					config.set_value(
						str("keybinds", i),
						event,
						JSON.stringify(serialize_keybind(key))
					)
	config.save(SETTINGS_FILE_PATH)
	print("saved keybind settings")

## Takes in an InputEvent and parses it into a storable dictionary 
func serialize_keybind(key: InputEvent) -> Dictionary:
	if key is InputEventJoypadButton:
		return {"JoypadButton": key.button_index}
	elif key is InputEventJoypadMotion:
		return {"JoypadMotion": key.axis}
	elif key is InputEventKey:
		return {"Key": key.keycode}
	return {"Unknown": 0}

## Takes in a stringified dictionary, parses it into a string, then creates an 
## input event with the information stored. 
func parse_keybind(data: String, device: int) -> InputEvent:
	var input = "UNBOUND"
	var json = JSON.parse_string(data) 
	if json.has("JoypadButton"):
		input = InputEventJoypadButton.new()
		input.button_index = json["JoypadButton"]
	elif json.has("JoypadMotion"):
		input = InputEventJoypadMotion.new()
		input.axis = json["JoypadMotion"]
	elif json.has("InputEventKey"):
		input = InputEventKey.new()
		input.keycode = json["InputEventKey"]
	
	if typeof(input) == TYPE_OBJECT : input.device = device 
	return input
		
		
func load_all_keybind_settings():
	var keybind_settings = []
	for i in 8:
		keybind_settings.append(load_settings_helper(str("keybinds", i)))
		for action in keybind_settings[i]:
			keybind_settings[i][action] = parse_keybind(keybind_settings[i][action], i)
	return keybind_settings 
