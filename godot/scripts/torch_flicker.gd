extends AnimatedSprite2D

@onready var point_light: PointLight2D = $"../PointLight2D2"

var new_energy: float
## Called when the node enters the scene tree for the first time.
func _ready() -> void:
	if self.is_visible_in_tree():
		new_energy = randf_range(4.0, 6.5)

## Called when the frame changes.
## Alters the energy of the torch to simulate flickering.
func _on_frame_changed() -> void:
	if self.frame != 0 && self.frame % 10 == 0:
		new_energy = randf_range(4.0, 6.5)
	elif point_light.energy > new_energy:
		point_light.energy -= (self.frame % 10) * 0.008
	elif point_light.energy <= new_energy:
		point_light.energy += (self.frame % 10) * 0.008
