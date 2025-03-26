extends Player
@onready var direct_sight: RayCast2D = $RayCast2D
@onready var nearby_players = []

func _process(_delta: float) -> void:
	for player in nearby_players:
		direct_sight.target_position = to_local(player.position)
		if direct_sight.is_colliding() && direct_sight.get_collider() == player:
			player.make_player_visible(self.get_player_id())
		else:
			player.make_player_invisible(self.get_player_id())

## This function is called when the animation_finished signal is emitted.
## It will update the animation finished field in the player class
func _on_owner_vis_animation_finished() -> void:
	self.set_anim_finished()

## This function tracks nearby metal objects and adds them to the player's list of metal objects
func _on_area_2d_body_entered(body: Node2D) -> void:
	if body is MetalObject:
		self.add_metal_object((body as MetalObject))
	elif body.has_method("get_player_id") && body != self:
		var player = body as Player
		nearby_players.append(player)

	
## This function removes nearby metal objects from the player's list of metal objects
func _on_area_2d_body_exited(body: Node2D) -> void:
	if body.has_method("is_metal"):
		self.remove_metal_object(body)
	elif body.has_method("get_player_id") && body != self:
		var player = body as Player
		player.make_player_invisible(self.get_player_id())


func _on_player_range_body_entered(body: Node2D) -> void:
	if body.has_method("get_player_id") && body != self:
		var player = body as Player
		self.add_nearby_player(player)


func _on_player_range_body_exited(body: Node2D) -> void:
	if body.has_method("get_player_id") && body != self:
		var player = body as Player
		self.remove_nearby_player(player)
