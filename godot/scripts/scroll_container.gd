extends ScrollContainer

@onready var scroll_container: ScrollContainer = $"."

func _process(delta):
	if scroll_container.is_visible_in_tree():
		if Input.is_action_pressed("scroll_up"):
			scroll_container.scroll_vertical -= 10
		elif Input.is_action_pressed("scroll_down"):
			scroll_container.scroll_vertical += 10
		elif Input.is_action_pressed("scroll_left"):
			scroll_container.scroll_horizontal -= 10
		elif Input.is_action_pressed("scroll_right"):
			scroll_container.scroll_horizontal += 10
		
