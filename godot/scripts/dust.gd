## Handles logic for showing the dust animation
##
## @author Charles Barth
## @version Spring 2025
extends AnimatedSprite2D

## When the visibility of the dust is changed, play the dust animation
## The animation fades out the dust particles
func _on_visibility_changed() -> void:
	if self.visible:
		var dust_player = self.get_child(0) as AnimationPlayer
		self.play("dust")
		dust_player.play("dust")

## When the dust animation is finished, hide the dust particles
func _on_animation_finished() -> void:
	self.visible = false
