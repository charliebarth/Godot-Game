extends Label

func _process(_delta: float) -> void:
	text = "Is Server: %s" % multiplayer.is_server()
