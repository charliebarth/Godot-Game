extends AnimatedSprite2D

func _on_animation_finished() -> void:
	if self.animation == "jump_fall":
		self.play("fall")
