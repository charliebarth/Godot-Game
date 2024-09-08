extends Player

# Assuming you have a Line2D node as a child of the player for drawing lines
@onready var line_drawer: Line2D = $LineDrawer

func _ready():
	var area = $Area2D
	#area.connect("area_entered", self, "_on_Area2D_area_entered")
	#area.connect("area_exited", self, "_on_Area2D_area_exited")
	area.connect("area_entered", Callable(self, "_on_Area2D_area_entered"))
	area.connect("area_exited", Callable(self, "_on_Area2D_area_exited"))

var metals_in_range = []

func _on_Area2D_area_entered(area: Area2D):
	print("Found Something")
	if area.has_method("get_weight"):  # Check if the area is a metal object
		print("Found Metal")
		metals_in_range.append(area)

func _on_Area2D_area_exited(area: Area2D):
	if area in metals_in_range:
		metals_in_range.erase(area)

#func _process(delta: float) -> void:
	#update_lines_to_metals()

func _draw():
	for metal in metals_in_range:
		if metal.is_inside_tree() and metal.is_metal():  # Check if the metal object is still valid
			draw_line(global_position, metal.global_position, Color.BLUE, 2)

#func update_lines_to_metals() -> void:
	#line_drawer.points = []
	#for metal in metals_in_range:
		##if metal.is_inside_tree():  # Check if the metal object is still valid
			##line_drawer.points.append(global_position)
			##line_drawer.points.append(metal.global_position)
		##if metal.is_metal:  # Additional check, might be redundant depending on your structure
			##line_drawer.draw_line(self.position, metal.position, Color.BLUE, 2)

#func apply_allomantic_force(metal: Node, push: bool) -> void:
	#var direction = (metal.global_position - global_position).normalized()
	#var distance = global_position.distance_to(metal.global_position)
	#var force = calculate_force(distance, metal.get_weight())
	#if push:
		#metal.apply_central_impulse(direction * force)
	#else:
		#pass
		##apply_central_impulse(-direction * force)  # Apply force to the player
#
#func calculate_force(distance: float, weight: float) -> float:
	#return 1000.0 * weight / max(distance, 1.0)  # Simple force calculation
	
	## Player.gd
#func _process(delta):
	#var metals_in_range = []
	#for body in $Area2D.get_overlapping_bodies():
		#if body.has_method("is_metal"):  # Checks if the body is a metal
			#metals_in_range.append(body)
#
	#update_lines_to_metals(metals_in_range)
#
#func update_lines_to_metals(metals):
	## Clear existing lines (if any)
	#$LineDrawer.clear()
#
	## Draw new lines
	#for metal in metals:
		#if metal.is_metal:  # Additional check, might be redundant depending on your structure
			#$LineDrawer.draw_line(self.position, metal.position, Color.blue, 2)
