extends AnimatedSprite2D

@onready var footstep = $"../Footstep" as AudioStreamPlayer2D
@onready var landing = $"../Landing" as AudioStreamPlayer2D
@onready var jump = $"../Jump" as AudioStreamPlayer2D
@export var previous_animation = ""

## This function is called when the animation_finished signal is emitted.
## This will transition from player from the jump fall transition animation to the fall animation
func _on_animation_finished() -> void:
	if self.animation == "jump_fall":
		self.play("fall")

## This function is called every frame and is used to player the footstep sound effect
func _process(_delta: float) -> void:
	if self.animation == "run" && (self.frame == 0 || self.frame == 5):
		footstep.play()

## This function is called when the animation_change signal is emmited.
## It is used to swap from jump to jump_fall transition when going from jump to fall
## This also plays the sound effects for jumping and landing
func _on_animation_changed() -> void:
	if self.animation == "fall" && previous_animation == "jump":
		self.play("jump_fall")
	previous_animation = self.animation

	if self.animation == "jump":
		jump.play()
	elif self.animation == "land":
		landing.play()
