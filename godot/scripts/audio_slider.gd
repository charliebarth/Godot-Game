## Handles an audio slider in the settings menu tab
##
## @author Charles Barth
## @version Spring 2025
extends Control

## The name of the audio
@onready var audio_name: Label = $HBoxContainer/AudioName
## The audio number corresponding to this audio
@onready var audio_num: Label = $HBoxContainer/AudioNum
## The hslider corresponding to this audio
@onready var h_slider: HSlider = $HBoxContainer/HSlider

## The bus name 
@export_enum("Master", "Music", "SFX", "PlayerSFX", "WorldSFX") var bus_name: String

## The index of the bus
var bus_index: int = 0

## This function is called when the node is added to the scene for the first time.
## It will set the audio name, audio number, and starting volume.
func _ready() -> void:
	get_bus_by_inedx()
	set_audio_name()
	set_audio_num()
	set_starting_volume()

## This function sets the audio name to the bus name.
func set_audio_name() -> void:
	audio_name.text = str(bus_name) + " Volume"

## This function sets the audio number to the bus volume.
func set_audio_num() -> void:
	audio_num.text = str(h_slider.value * 100) + "%"

## This function sets the starting volume to the saved volume.
func set_starting_volume() -> void:
	var audio_settings = ConfigFileHandler.load_settings_helper("audio")
	h_slider.value = audio_settings[bus_name]

## This function is called when the slider value is changed.
## It will set the audio number and set the bus volume.
func _on_h_slider_value_changed(value: float) -> void:
	set_audio_num()
	AudioServer.set_bus_volume_db(bus_index, linear_to_db(value))

## This function gets the bus index by the bus name.
func get_bus_by_inedx() -> void:
	bus_index = AudioServer.get_bus_index(bus_name)

## When the apply button is pressed, save the audio setting.
func _on_apply_pressed() -> void:
	var volume = db_to_linear(AudioServer.get_bus_volume_db(bus_index))
	ConfigFileHandler.save_audio_setting(bus_name, volume)
