## Handles the logic for the Settings tab menu
## @author Charles Barth
## @version Spring 2025
extends Control

## The associated tab container
@onready var tab_container: TabContainer = $TabContainer


## When the visibility of the settings tab menu is changed, grab the focus
## This allows controllers to navigate the settings menu
func _on_visibility_changed() -> void:
	if self.visible:
		tab_container.get_tab_bar().grab_focus()
