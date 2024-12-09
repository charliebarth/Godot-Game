extends Control

@onready var tab_container: TabContainer = $TabContainer

func _on_visibility_changed() -> void:
	if self.visible:
		tab_container.get_tab_bar().grab_focus()
