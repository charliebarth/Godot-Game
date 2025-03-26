extends Node
## Handles the logic for saving and loading to and from the config file. 
## 
## @author Charles Barth 
## @author Trinity Pittman 
## @version Spring 2025

## A reference to the config file
var config = ConfigFile.new()
## The path to the config file 
const SETTINGS_FILE_PATH = "user://settings.ini"

## Stores the resolution types 
var resolutions = [Vector2i(1920, 1080), Vector2i(1920,1200), 
				Vector2i(1080,960), Vector2i(1280, 720), Vector2i(800,600)]
## Stores the action types 
var actions = ["jump", "sprint", "roll", "attack", "throw", "low_burn", 
				"pewter", "iron", "steel"]


## Called when the node enters the scene tree for the first time. Sets the 
## values that should be in the config file if it doesn't exist. 
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
		config.set_value("ui", "pos", 0)
		
		var keybind = ["{\"JoypadButton\":0}", "{\"JoypadButton\":7}", "{\"JoypadMotion\":5}", 
		"{\"JoypadButton\":2}", "{\"JoypadButton\":14}", "{\"JoypadButton\":3}", 
				"{\"JoypadButton\":9}", "{\"JoypadMotion\":4}", "{\"JoypadMotion\":5}"]
		for i in 8:
			for j in 9:
				config.set_value(str("keybinds", i), actions[j], keybind[j]);
	
		config.save(SETTINGS_FILE_PATH)
	else:
		config.load(SETTINGS_FILE_PATH)


## Loads the setting based on the string passed in, for example "audio" or "ui".
## 
## @param type The name of the section to load the settings for
## @returns A dictionary of the settings for the specified section
func load_settings_helper(type: String) -> Dictionary:
	var settings = {}
	for key in config.get_section_keys(type):
		settings[key] = config.get_value(type, key)
	return settings


## Save the audio setting to the config file.
##
## @param key The name of the audio setting being saved
## @param value The value it is being saved as
func save_audio_setting(key: String, value: float) -> void:
	config.set_value("audio", key, value)
	config.save(SETTINGS_FILE_PATH)


## Saves the graphics settings to the config file. 
func save_graphics_setting() -> void:
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

## Saves the UI settings to the config file. 
## 
## @param size The size to save 
## @param opacity The opacity to save 
## @param pos The position index to save 
func save_ui_settings(size: float, opacity: float, pos: int) -> void:
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
		"pos",
		pos
	)
	
	config.save(SETTINGS_FILE_PATH)


## Saves the keybind settings for every player to the config file
func save_keybind_settings() -> void:
	for i in 8: # There are 8 players
		for action in actions:	# Go through the list of actions 
			for key in InputMap.action_get_events(action): # Thru bound keys 
				if key.device == i:		# Find devices bound key
					config.set_value(
						str("keybinds", i),
						action,
						JSON.stringify(serialize_keybind(key))
					)
	config.save(SETTINGS_FILE_PATH)


## Takes in an InputEvent and parses it into a storable dictionary 
##
## @param key The input event to serialize 
## @returns The serialized input event 
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
## 
## @param data The stringified dictionary representing a key to parse
## @param device The players device id 
## @returns The parsed input event 
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


## Loads the keybind settings from the config file
##
## @returns A dictionary of the keybind settings 
func load_all_keybind_settings() -> Array:
	var keybind_settings = []
	for i in 8:
		keybind_settings.append(load_settings_helper(str("keybinds", i)))
		for action in keybind_settings[i]:
			keybind_settings[i][action] = parse_keybind(keybind_settings[i][action], i)
	return keybind_settings 
