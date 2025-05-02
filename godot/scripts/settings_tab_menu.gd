extends Control

@onready var tab_container: TabContainer = $TabContainer

@onready var sound: TabBar = $TabContainer/Sound
@onready var graphics: TabBar = $TabContainer/Graphics
@onready var ui_customization: TabBar = $"TabContainer/UI Customization"


## When the visibility of the settings tab menu is changed, grab the focus
## This allows controllers to navigate the settings menu
func _on_visibility_changed() -> void:
	if self.is_visible_in_tree():
		tab_container.get_tab_bar().grab_focus()
