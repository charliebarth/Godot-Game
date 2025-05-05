extends Player
@onready var nearby_players = []

## This function is called when the animation_finished signal is emitted.
## It will update the animation finished field in the player class
func _on_owner_vis_animation_finished() -> void:
	self.set_anim_finished()

## This function tracks nearby metal objects and adds them to the player's list of metal objects
func _on_area_2d_body_entered(body: Node2D) -> void:
	if body is MetalObject:
		self.add_metal_object((body as MetalObject))

	
## This function removes nearby metal objects from the player's list of metal objects
func _on_area_2d_body_exited(body: Node2D) -> void:
	if body.has_method("is_metal"):
		self.remove_metal_object(body)


## This function adds a nearby player to the player's list of nearby players
##
## @param `body` (Node2D) - The body that entered the player's range
func _on_player_range_body_entered(body: Node2D) -> void:
	if body.has_method("get_player_id") && body != self:
		var player = body as Player
		self.add_nearby_player(player)


## This function removes a nearby player from the player's list of nearby players
##
## @param `body` (Node2D) - The body that exited the player's range
func _on_player_range_body_exited(body: Node2D) -> void:
	if body.has_method("get_player_id") && body != self:
		var player = body as Player
		self.remove_nearby_player(player)
