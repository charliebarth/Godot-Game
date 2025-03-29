extends Control

@onready var tab_container: TabContainer = $TabContainer

@onready var sound: TabBar = $TabContainer/Sound
@onready var graphics: TabBar = $TabContainer/Graphics
@onready var controls: Controls = $TabContainer/Controls
@onready var ui_customization: TabBar = $"TabContainer/UI Customization"


## When the visibility of the settings tab menu is changed, grab the focus
## This allows controllers to navigate the settings menu
func _on_visibility_changed() -> void:
	if self.visible:
		tab_container.get_tab_bar().grab_focus()


# TODO poential solution to inability to navigate tab bar 
func _on_tab_container_tab_changed(tab: int) -> void:
	#var to_focus = null
	#match tab:
		#0: to_focus = null
		#1: to_focus = null
		#2: to_focus = null
		#3: to_focus = $"TabContainer/UI Customization/MarginContainer/ScrollContainer/VBoxContainer/UI_Placement/VBoxContainer/PanelContainer/GridContainer/0"
	#
	#if to_focus != null:
		#to_focus.grab_focus()
	#print(to_focus)
	pass
		
