## Handles logic for candle lighting
## 
## @author Charles Barth
## @version Spring 2025
extends AnimatedSprite2D

@onready var point_light: PointLight2D = $"../PointLight2D"

var new_energy: float

## When the candle is ready, set the new energy to a random value between 2.5 and 4.0
## and wait for a random amount of time between 0.0 and 1.2 seconds
## then start playing the candle flame animation
func _ready() -> void:
	if self.is_visible_in_tree():
		var min_energy: float = 2.5
		var max_energy: float = 4.0
		var min_delay: float = 0.0
		var max_delay: float = 1.2

		new_energy = randf_range(min_energy, max_energy)
		var delay = randf_range(min_delay, max_delay)
		await get_tree().create_timer(delay).timeout # Wait for the delay
		play("candleFlame") # Start playing the animation

## When the frame changes, update the energy and speed scale of the candle
func _on_frame_changed() -> void:
	var min_energy: float = 2.5
	var max_energy: float = 4.0
	var min_speed_scale: float = 0.8
	var max_speed_scale: float = 1.5
	var num_frames: int = 10
	var min_energy_change: float = 0.008

	if self.frame != 0 && self.frame % num_frames == 0:
		new_energy = randf_range(min_energy, max_energy)
	elif point_light.energy > new_energy:
		point_light.energy -= (self.frame % num_frames) * min_energy_change
	elif point_light.energy <= new_energy:
		point_light.energy += (self.frame % num_frames) * min_energy_change
	
	if self.frame != 0 && self.frame % 15 == 0:
		self.speed_scale = randf_range(min_speed_scale, max_speed_scale)
