extends AnimatedSprite2D

@onready var footstep = $"../Footstep" as AudioStreamPlayer2D
@onready var landing = $"../Landing" as AudioStreamPlayer2D
@onready var jump = $"../Jump" as AudioStreamPlayer2D
@export var previous_animation = ""

func _on_animation_finished() -> void:
	if self.animation == "jump_fall":
		self.play("fall")

func _process(_delta: float) -> void:
	if self.animation == "run" && (self.frame == 0 || self.frame == 5):
		footstep.play()


func _on_animation_changed() -> void:
	if self.animation == "fall" && previous_animation == "jump":
		self.play("jump_fall")
	previous_animation = self.animation

	if self.animation == "jump":
		jump.play()
	elif self.animation == "land":
		landing.play()
