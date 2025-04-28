## Allows controllers to scroll in the UI menus
##
## @author Trinity Pittman 
extends ScrollContainer

## The scroll container to allow scrolling in
@onready var scroll_container: ScrollContainer = $"."

## Called every visual frame. If the scroll conatiner is visible, and scrolling
## is detected, move the container in the corresponding direction. 
## 
## @param `delta` (int) - The time since the last frame.
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
