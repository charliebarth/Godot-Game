## Signal for player id changed
## 
## @author Charles Barth
## @version Spring 2025
extends PointLight2D

## This function is called when the player id is changed
## This will update the cull mask so that the light only affects the player
func _on_player_id_changed() -> void:
	var player: Player = self.get_parent() as Player
	var layer_num = (player.get_player_id() * 2)
	self.range_item_cull_mask = 1 << layer_num
