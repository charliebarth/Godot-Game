## The logic for the lime selector 
##
## @author Charles Barth
## @version Spring 2025
extends Sprite2D

# Variable for setting the lerp speed
var lerp_speed = 15.0
@onready var player = self.get_parent() as Player

## This function moves the line selector based on the joystick input
func _process(delta):
	var radius = 40

	# Get the joystick direction
	var joystick_x = Input.get_joy_axis(player.get_device_id(), JOY_AXIS_RIGHT_X)
	var joystick_y = Input.get_joy_axis(player.get_device_id(), JOY_AXIS_RIGHT_Y)
	
	# Calculate the direction vector and invert y-axis
	var direction = Vector2(joystick_x, joystick_y) # Negate y for intuitive direction

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
