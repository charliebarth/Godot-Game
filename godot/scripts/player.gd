extends Player

## This function is called when the animation_finished signal is emitted.
## It will update the animation finished field in the player class
func _on_owner_vis_animation_finished() -> void:
	self.set_anim_finished()

## This function tracks nearby metal objects and adds them to the player's list of metal objects
func _on_area_2d_body_entered(body: Node2D) -> void:
	if body.has_method("is_metal"):
		var metal = body as MetalObject
		self.add_metal_object(metal)

## This function removes nearby metal objects from the player's list of metal objects
func _on_area_2d_body_exited(body: Node2D) -> void:
	if body.has_method("is_metal"):
		var metal = body as MetalObject
		self.remove_metal_object(metal)
