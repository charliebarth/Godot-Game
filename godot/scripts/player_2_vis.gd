extends AnimatedSprite2D

@export var previous_animation = ""

func _on_animation_finished() -> void:
	if self.animation == "jump_fall":
		self.play("fall")


func _on_player_id_changed() -> void:
	var player = $".." as Player
	var name_as_str: String = self.name as String
	var player_num: int = name_as_str[-4].to_int()
	var player_ids = [1, 2, 3, 4]
	
	player_ids.erase(player.get_player_id())
	
	var layer_num: int = (player_ids[player_num - 1] * 2) - 1
	
	self.visibility_layer = 1 << layer_num
	self.light_mask = 1 << layer_num


func _on_animation_changed() -> void:
	if self.animation == "fall" && previous_animation == "jump":
		self.play("jump_fall")
	previous_animation = self.animation
