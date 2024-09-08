extends Player

@onready var _animated_sprite = $AnimatedSprite2D

func _process(_delta):
	pass
	#if Input.is_action_just_pressed("jump"):
		#_animated_sprite.play("jump")
		##_animated_sprite.play("jump_fall")
		##_animated_sprite.play("fall")
		##_animated_sprite.play("landing")
	#
	#var horizontal_dir = Input.get_axis("move_left", "move_right")
	#if horizontal_dir != 0:
		#_animated_sprite.scale.x = sign(horizontal_dir)
		#if is_on_floor():
			#_animated_sprite.play("run_right")
	#elif horizontal_dir == 0 && is_on_floor():
		#_animated_sprite.stop()
