extends Sprite2D

# Variable for setting the lerp speed
var lerp_speed = 10.0

func _ready() -> void:
	var player: Player = self.get_parent() as Player
	var layer_num = (player.get_player_id() * 2)
	
	self.visibility_layer = 1 << layer_num
	self.light_mask = 1 << layer_num | 1

func _process(delta):
	var radius = 65
	# Get the joystick direction
	var joystick_x = Input.get_axis("right_stick_left", "right_stick_right")
	var joystick_y = Input.get_axis("right_stick_down", "right_stick_up")
	
	# Calculate the direction vector and invert y-axis
	var direction = Vector2(joystick_x, -joystick_y)  # Negate y for intuitive direction

	# Only update if the joystick is being pushed
	if direction.length() > 0.0:
		# Normalize direction to maintain consistent radius
		direction = direction.normalized()
		
		# Calculate the current direction of the arrow relative to the player
		var current_direction = self.position.normalized()
		
		# Smoothly interpolate the direction
		var smooth_direction = current_direction.lerp(direction, lerp_speed * delta).normalized()
		
		# Set the arrow's position to always be at the fixed radius distance
		self.position = smooth_direction * radius
		
		# Calculate the angle for the arrow to face the direction smoothly
		var angle = smooth_direction.angle()
		self.rotation = lerp_angle(self.rotation, angle, lerp_speed * delta)
