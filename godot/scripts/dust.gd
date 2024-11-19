extends AnimatedSprite2D

func _ready() -> void:
	var player: Player = self.get_parent() as Player
	var layer_num = (player.get_player_id() * 2)
	
	self.visibility_layer = 1 << layer_num
	self.light_mask = 1 << layer_num | 1
	self.visible = false

func _on_visibility_changed() -> void:
	if self.visible:
		print("hello")
		var dust_player = self.get_child(0) as AnimationPlayer
		self.play("dust")
		dust_player.play("dust")


func _on_animation_finished() -> void:
	self.visible = false
