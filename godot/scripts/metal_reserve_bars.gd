extends MetalReserveBarManager


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var player: Player = self.get_parent() as Player	
	var layer_num = (player.get_player_id() * 2)
	
	for child in self.get_children():
		child.visibility_layer = 1 << layer_num
		child.light_mask = 1 << layer_num
