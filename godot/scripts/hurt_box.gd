extends Area2D

# Reference to the parent player
@onready var parent_player = get_parent() as Player

# Handle when another Area2D enters this hurtbox
func _on_area_entered(area: Area2D):
	# Check if the area belongs to a hitbox
	if area.name == "Hitbox" && area.get_parent() != get_parent():
		# Get the node owning the hitbox
		var attacker = area.get_parent() 
		# Make sure we don't damage ourselves
		if attacker != parent_player:
			# Deal damage
			parent_player.adjust_health(-10.0)
			if parent_player.get_health() <= 0:
				# increment the elims of the attacking player
				area.get_parent().increment_eliminations()
				print("Attacker's kills: ", area.get_parent().get_eliminations())
