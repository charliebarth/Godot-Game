## Handles button actions for the new game menu. 
##
## @author Trinity Pittman
extends Control

## The buttons corresponding to each map 
@onready var map_one: TextureButton = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel/VBoxContainer/TextureButton1
@onready var map_two: TextureButton = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel2/VBoxContainer2/TextureButton2
@onready var map_three: TextureButton = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel3/VBoxContainer2/TextureButton3
## The panels behind each map button
@onready var panel_1: Panel = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel
@onready var panel_2: Panel = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel2
@onready var panel_3: Panel = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer/Panel3
## The buttons corresponding to the game modes
@onready var game_mode_1: Button = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer2/VBoxContainer/Button
@onready var game_mode_2: Button = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer2/VBoxContainer3/Button
@onready var game_mode_3: Button = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer2/VBoxContainer2/Button
## The check button for solor or team mode 
@onready var check_button: CheckButton = $PanelContainer/MarginContainer/ScrollContainer/VBoxContainer/HBoxContainer3/CheckButton
## The textures for the panels when a map button is pressed/unpressed
const MAP_PRESSED = preload("res://assets/Maps/map_pressed.tres")
const MAP_UNPRESSED = preload("res://assets/Maps/map_unpressed.tres")
## A reference to the game
@onready var game = get_node("/root/Game") as Game

## Stores the panels
var panels = []
## The group for the map buttons
var map_button_group = ButtonGroup.new()
## The group for the game mode buttons 
var mode_button_group = ButtonGroup.new()

## Called when this node is added to the scene tree. Adds buttons to their 
## groups, and connects signals to functions. 
func _ready() -> void:
	# Add map buttons to their own group
	map_one.button_group = map_button_group
	map_two.button_group = map_button_group
	map_three.button_group = map_button_group
	# Add game mode buttons to their own group
	game_mode_1.button_group = mode_button_group
	game_mode_2.button_group = mode_button_group
	game_mode_3.button_group = mode_button_group
	# Connect signals to the button groups
	map_button_group.connect("pressed", on_map_button_press)
	mode_button_group.connect("pressed", on_mode_button_press)
	# Add panels to the panel array 
	panels.append(panel_1)
	panels.append(panel_2)
	panels.append(panel_3)

## When a map button is pressed, check which button it was and set the map in 
## the game. 
## 
## @param `button` (BaseButton) - The button that was pressed. 
func on_map_button_press(button: BaseButton) -> void:
	# Find which button was pressed.
	var map = ""
	match button:
		map_one:
			map = "MapOne"
		map_two:
			map = "MapTwo"
		map_three:
			map = "MapThree"
	
	# Set the map in the game. 
	game.set_game_map(map)
	# Set the panel behind the button to the pressed texture. 
	for i in range(0, len(map_button_group.get_buttons())):
		if map_button_group.get_buttons()[i].button_pressed:
			panels[i].add_theme_stylebox_override("panel", MAP_PRESSED)
		else:
			panels[i].add_theme_stylebox_override("panel", MAP_UNPRESSED)
	
	
# NOTE - Remember that the other two game modes are currently disabled 
## When the game mode button is pressed, find which button it was and set the 
## game mode in the game. 
## 
## @param `button` (BaseButton) - The button that was pressed. 
func on_mode_button_press(button: BaseButton) -> void:
	var mode = ""
	match button:
		game_mode_1:
			mode = "Last Player Standing"
		game_mode_2:
			mode = "Head Hunters"
		game_mode_3:
			mode = "Capture the Flag"
	game.set_game_mode(mode)
		

## When the check button is toggled, sets the game mode to solo or team based on
## whether the button is toggled off or on.
##
## @param `toggled_on` (bool) - false is solo, true is team. 
func _on_check_button_toggled(toggled_on: bool) -> void:
	game.set_team_game(toggled_on)
