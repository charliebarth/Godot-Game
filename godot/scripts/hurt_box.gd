extends Area2D

# Reference to the parent player
@onready var parent_player = get_parent() as Player

# Handle when another Area2D enters this hurtbox
func _on_area_entered(area):
	# Check if the area belongs to a hitbox
	if area.is_in_group("Hitbox"):
		# Get the node owning the hitbox
		var attacker = area.get_parent() 
		# Make sure we don't damage ourselves
		if attacker != parent_player:
			# Deal damage
			parent_player.take_damage(10) 
