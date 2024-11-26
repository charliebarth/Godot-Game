extends AnimatedSprite2D


func _on_visibility_changed() -> void:
	if self.visible:
		var dust_player = self.get_child(0) as AnimationPlayer
		self.play("dust")
		dust_player.play("dust")


func _on_animation_finished() -> void:
	self.visible = false
