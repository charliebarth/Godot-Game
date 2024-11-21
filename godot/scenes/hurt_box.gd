extends Area2D

# Reference to the parent player
var parent_player = null

func _ready():
	# Connect the area_entered signal to handle collisions
	var hurt_box = get_node("HurtBox")
	hurt_box.connect("area_entered", _on_area_entered)

# Set the parent player (called during initialization)
func set_parent_player(player):
	parent_player = player

# Handle when another Area2D enters this hurtbox
func _on_area_entered(area):
	# Check if the area belongs to a hitbox
	if area.is_in_group("Hitbox"):
		var attacker = area.get_parent() # Get the node owning the hitbox
		if attacker != parent_player:
			# Make sure it's not the player owning this hurtbox
			if attacker.has_method("get_melee_damage"):
				var damage = attacker.get_melee_damage() # Get the attack damage
				parent_player.take_damage(damage) # Apply damage to the parent player
