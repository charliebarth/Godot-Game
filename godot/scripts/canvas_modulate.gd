extends CanvasModulate


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var player: Player = self.get_parent() as Player
	var layer_num = (player.get_player_id() * 2)
	
	self.visibility_layer = 1 << layer_num
	self.light_mask = 1 << layer_num
