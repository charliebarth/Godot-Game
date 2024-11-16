extends Node2D


# Called when the node enters the scene tree for the first time.
func _draw() -> void:
	var player: Player = self.get_parent() as Player
	var points = player.get_line_points()
	
	if points.is_empty():
		return
	
	var player_pos = to_local(self.get_global_position())
	var line_points = PackedVector2Array()
	for target_point in points:
		line_points.append(player_pos)
		line_points.append(to_local(target_point))
	
	var color = Color(0.117647, 0.564706, 1, 0.7)
	draw_multiline(line_points, color, 2.0)
	
