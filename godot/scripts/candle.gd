extends AnimatedSprite2D

@onready var point_light: PointLight2D = $"../PointLight2D"

var new_energy: float


func _ready() -> void:
	if self.is_visible_in_tree():
		new_energy = randf_range(0.4, 1.0)
		var delay = randf_range(0.0, 1.2)
		await get_tree().create_timer(delay).timeout # Wait for the delay
		play("candleFlame") # Start playing the animation

func _on_frame_changed() -> void:
	if self.frame != 0 && self.frame % 10 == 0:
		new_energy = randf_range(0.4, 1.1)
	elif point_light.energy > new_energy:
		point_light.energy -= (self.frame % 10) * 0.008
	elif point_light.energy <= new_energy:
		point_light.energy += (self.frame % 10) * 0.008
	
	if self.frame != 0 && self.frame % 15 == 0:
		self.speed_scale = randf_range(0.8, 1.5)
