extends Control

@onready var audio_name: Label = $HBoxContainer/AudioName
@onready var audio_num: Label = $HBoxContainer/AudioNum
@onready var h_slider: HSlider = $HBoxContainer/HSlider

@export_enum("Master", "Music", "SFX") var bus_name: String

var bus_index: int = 0

func _ready() -> void:
	get_bus_by_inedx()
	set_audio_name()
	set_audio_num()
	set_starting_volume()


func set_audio_name() -> void:
	audio_name.text = str(bus_name) + " Volume"

func set_audio_num() -> void:
	audio_num.text = str(h_slider.value * 100) + "%"

func set_starting_volume() -> void:
	var audio_settings = ConfigFileHandler.load_audio_settings()
	print(audio_settings[bus_name])
	h_slider.value = audio_settings[bus_name]

func _on_h_slider_value_changed(value: float) -> void:
	set_audio_num()
	AudioServer.set_bus_volume_db(bus_index, linear_to_db(value))

func get_bus_by_inedx() -> void:
	bus_index = AudioServer.get_bus_index(bus_name)


func _on_apply_pressed() -> void:
	var volume = db_to_linear(AudioServer.get_bus_volume_db(bus_index))
	ConfigFileHandler.save_audio_setting(bus_name, volume)
