extends Area2D

# Reference to the parent player
@onready var parent_player = get_parent() as Player

# Handle when another Area2D enters this hurtbox
func _on_area_entered(area: Area2D):
	if parent_player.is_remote_player():
		return
	
	# Check if the area belongs to a hitbox
	if (area.name == "RightHitbox" || area.name == "LeftHitbox") && area.get_parent() != get_parent():
		# Get the node owning the hitbox
		var attacker = area.get_parent() as Player
		# Make sure we don't damage ourselves
		if attacker != parent_player:
			# Deal damage
			var damage = -10.0
			if parent_player.is_burning_metal_from_string("pewter"):
				damage = damage * 1.35
			
			parent_player.adjust_health(damage)
			if parent_player.get_health() <= 0:
				# increment the elims of the attacking player
				area.get_parent().increment_eliminations(attacker.get_player_id())
