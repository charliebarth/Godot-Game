extends Control

@export var bkg_color: Color
@export var line_color: Color
@export var highlight_color: Color

@export var outer_radius: int = 256
@export var inner_radius: int = 64
@export var line_width: int = 4

@export var options: Array[String]
@export var section_colors: Array[Color]

#@onready var player: Player = $"."

var selected_index: int = 0

func _draw():
	draw_circle(Vector2.ZERO, outer_radius, bkg_color)
	draw_arc(Vector2.ZERO, inner_radius, 0, TAU, 128, line_color, line_width, true)
	
	if len(options) >= 3:
		
		for i in range(len(options)):
			# Draw the seperator lines
			var rads = TAU * i / (len(options))
			var point = Vector2.from_angle(rads)
			draw_line(
				point*inner_radius, 
				point*outer_radius,
				line_color,
				line_width,
				true
			)
	
			# Draw the text 
			var start_rads = (TAU * (i-1)) / (len(options))
			var end_rads = (TAU * (i)) / (len(options))
			var mid_rads = (start_rads + end_rads) / 2.0 * - 1
			var radius_mid = (inner_radius + outer_radius) / 2.0
			
			var draw_pos = radius_mid * Vector2.from_angle(mid_rads)
			
			var font: Font = preload("res://assets/pixelated-times-new-roman.ttf")
			
			var size := font.get_string_size(options[i]) 
			
			# Draw highlight color 
			if selected_index == i:
				var points_per_arc = 32
				var points_inner = PackedVector2Array()
				var points_outer = PackedVector2Array()
				
				for j in range(points_per_arc + 1):
					var angle = start_rads + j * (end_rads - start_rads) / points_per_arc
					points_inner.append(inner_radius * Vector2.from_angle(TAU - angle))
					points_outer.append(outer_radius * Vector2.from_angle(TAU - angle))
					
				points_outer.reverse()
				draw_polygon(
					points_inner + points_outer,
					PackedColorArray([section_colors[i]])
				)

			draw_string(
				font,
				draw_pos - (size * 1.7) ,
				options[i],
				0, -1, 64
			)
			
			# Draw the color 
			var colors = [section_colors[1], section_colors[0], section_colors[5], section_colors[4], section_colors[3], section_colors[2]]
			draw_arc(Vector2.ZERO, outer_radius, start_rads, end_rads, 128, colors[i], line_width, true)
			#draw_arc(Vector2.ZERO, outer_radius, start_rads, end_rads, 128, section_colors[i], 8)
			
			

func _process(delta: float) -> void:
	var lerp_speed = 15.0
	var radius = 256


	# Get the joystick direction
	var joystick_x = Input.get_joy_axis(0, JOY_AXIS_RIGHT_X)
	var joystick_y = Input.get_joy_axis(0, JOY_AXIS_RIGHT_Y)
	
	# Calculate the direction vector and invert y-axis
	var direction = Vector2(joystick_x, joystick_y) # Negate y for intuitive direction

	# Only update if the joystick is being pushed
	if direction.length() > 0.9:
		# Normalize direction to maintain consistent radius
		direction = direction.normalized()
		print("Dir: ", direction)
		
		# Calculate the current direction of the arrow relative to the player
		var current_direction = self.position.normalized()
		print("Curr dir: ", current_direction)
		
		# Smoothly interpolate the direction
		var smooth_direction = current_direction.lerp(direction, lerp_speed * delta).normalized()
		print("Smooth dir: ", smooth_direction)
		
		# Set the arrow's position to always be at the fixed radius distance
		#self.position = smooth_direction * radius
		#
		## Calculate the angle for the arrow to face the direction smoothly
		#var angle = smooth_direction.angle()
		#self.rotation = lerp_angle(self.rotation, angle, lerp_speed * delta)
	#var move_delay = 500  # Time in seconds before allowing another movement
	#var time_since_last_move = 700  # Timer to track movement delay
	#
	#var controller_input = 0 
	## Get the joystick direction
	#var joystick_x = Input.get_joy_axis(0, JOY_AXIS_RIGHT_X)
	#var joystick_y = Input.get_joy_axis(0, JOY_AXIS_RIGHT_Y)
#
	## Calculate the direction vector and invert y-axis
	#var direction = Vector2(joystick_x, joystick_y) # Negate y for intuitive direction
#
	## Only update if the joystick is being pushed
	#if direction.length() > 0.0:
		#if time_since_last_move > move_delay:  # Check if the delay has passed
			#print("X: ", joystick_x)
			#print("Y: ", joystick_y)
			#if joystick_x < -0.9:
				#controller_input = -1 
			#elif joystick_x > 0.9:
				#controller_input = 1
		#time_since_last_move = 0.0
			#
	## Scroll through the options based on input
	#if controller_input != 0:
		#selected_index = (selected_index + controller_input) % len(options)
		#
	#time_since_last_move += delta
	## Redraw the wheel
	queue_redraw()

		
		
func close():
	hide()
	return options[selected_index]
	
func choose_option():
	print("Selected: ", options[selected_index])  # You can perform any action here
	#You can also trigger events or actions when an option is selected.
	
	
