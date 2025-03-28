extends Control
@onready var map_one: TextureButton = $PanelContainer/MarginContainer/VBoxContainer/HBoxContainer/Panel/VBoxContainer/TextureButton1
@onready var map_two: TextureButton = $PanelContainer/MarginContainer/VBoxContainer/HBoxContainer/Panel2/VBoxContainer2/TextureButton2
@onready var map_three: TextureButton = $PanelContainer/MarginContainer/VBoxContainer/HBoxContainer/Panel3/VBoxContainer2/TextureButton3

@onready var panel_1: Panel = $PanelContainer/MarginContainer/VBoxContainer/HBoxContainer/Panel
@onready var panel_2: Panel = $PanelContainer/MarginContainer/VBoxContainer/HBoxContainer/Panel2
@onready var panel_3: Panel = $PanelContainer/MarginContainer/VBoxContainer/HBoxContainer/Panel3

const MAP_PRESSED = preload("res://assets/Maps/map_pressed.tres")
const MAP_UNPRESSED = preload("res://assets/Maps/map_unpressed.tres")

@onready var game = get_node("/root/Game") as Game

var panels = []
var map_button_group = ButtonGroup.new()

func _ready() -> void:
	map_one.button_group = map_button_group
	map_two.button_group = map_button_group
	map_three.button_group = map_button_group
	
	map_button_group.connect("pressed", on_button_press)
	
	panels.append(panel_1)
	panels.append(panel_2)
	panels.append(panel_3)
	

func on_button_press(button: BaseButton) -> void:
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
	
	print(map)
