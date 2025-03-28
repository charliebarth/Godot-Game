extends Control
@onready var map_one: TextureButton = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel/VBoxContainer/TextureButton1
@onready var map_two: TextureButton = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel2/VBoxContainer2/TextureButton2
@onready var map_three: TextureButton = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel3/VBoxContainer2/TextureButton3

@onready var panel_1: Panel = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel
@onready var panel_2: Panel = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel2
@onready var panel_3: Panel = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel3

@onready var game_mode_1: Button = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer2/VBoxContainer/Button
@onready var game_mode_2: Button = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer2/VBoxContainer3/Button
@onready var game_mode_3: Button = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer2/VBoxContainer2/Button


const MAP_PRESSED = preload("res://assets/Maps/map_pressed.tres")
const MAP_UNPRESSED = preload("res://assets/Maps/map_unpressed.tres")

@onready var game = get_node("/root/Game") as Game

var panels = []
var map_button_group = ButtonGroup.new()
var mode_button_group = ButtonGroup.new()

func _ready() -> void:
	map_one.button_group = map_button_group
	map_two.button_group = map_button_group
	map_three.button_group = map_button_group
	
	game_mode_1.button_group = mode_button_group
	game_mode_2.button_group = mode_button_group
	game_mode_3.button_group = mode_button_group
	
	map_button_group.connect("pressed", on_map_button_press)
	mode_button_group.connect("pressed", on_mode_button_press)
	
	panels.append(panel_1)
	panels.append(panel_2)
	panels.append(panel_3)
	

func on_map_button_press(button: BaseButton) -> void:
	var map = ""
	match button:
		map_one:
			map = "MapOne"
		map_two:
			map = "MapTwo"
		map_three:
			map = "MapThree"
	
	game.set_game_map(map)
	for i in range(0, len(map_button_group.get_buttons())):
		if map_button_group.get_buttons()[i].button_pressed:
			panels[i].add_theme_stylebox_override("panel", MAP_PRESSED)
		else:
			panels[i].add_theme_stylebox_override("panel", MAP_UNPRESSED)
	
	
# TODO needs to be filled out with the game modes we will have 
func on_mode_button_press(button: BaseButton) -> void:
	var mode = ""
	match button:
		game_mode_1:
			mode = "Last Player Standing"
		game_mode_2:
			mode = ""
		game_mode_3:
			mode = ""
	game.set_game_mode(mode)
		
