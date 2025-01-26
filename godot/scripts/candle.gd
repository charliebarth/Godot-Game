extends Node2D

@onready var point_light: PointLight2D = $PointLight2D

var target_energy: float
var current_energy: float
var transition_time: float
var timer: Timer

func _ready() -> void:
	var animation = get_node("AnimatedSprite2D") as AnimatedSprite2D
	var delay = randf_range(0.0, 1.2)
	await get_tree().create_timer(delay).timeout # Wait for the delay
	var animation_name = "candleFlame" # Set the animation name
	animation.play(animation_name) # Start playing the animation
	flicker()

func flicker():
	print("flick")
	target_energy = randf_range(0.35, 0.9) # Pick a new energy level in the range
	current_energy = point_light.energy
	transition_time = randf_range(0.1, 0.3) # Time to transition to the new energy level
	timer = Timer.new()
	timer.wait_time = 0.01
	timer.one_shot = false
	timer.connect("timeout", Callable(self, "_update_light_energy"))
	add_child(timer)
	timer.start()

func _update_light_energy():
	var step = (target_energy - current_energy) / (transition_time / timer.wait_time)
	current_energy += step

	if abs(target_energy - current_energy) < 0.01:
		point_light.energy = target_energy
		timer.queue_free()
		# Start the next flicker after reaching the target
		flicker()
	else:
		point_light.energy = current_energy
