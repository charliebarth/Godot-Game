extends Player

## This function is called when the animation_finished signal is emitted.
## It will update the animation finished field in the player class
func _on_owner_vis_animation_finished() -> void:
	self.set_anim_finished()


func _on_steel_range_body_entered(body: Node2D) -> void:
	if body.has_method("is_metal"):
		var metal = body as MetalObject
		self.add_metal_object(metal)

func _on_steel_range_body_exited(body: Node2D) -> void:
	if body.has_method("is_metal"):
		var metal = body as MetalObject
		self.remove_metal_object(metal)
