extends Label

## This function is called every frame and is used to display the frames per second
func _process(_delta: float) -> void:
	text = "FPS: %s" % [Engine.get_frames_per_second()]
