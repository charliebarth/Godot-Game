extends PointLight2D

func _on_player_id_changed() -> void:
	var player: Player = self.get_parent() as Player
	var layer_num = (player.get_player_id() * 2)
	self.range_item_cull_mask = (1 << layer_num | 1 << (layer_num - 1))
