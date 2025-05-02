extends Camera2D

@onready var _1: Marker2D = $"../SpawnPoints/1"
@onready var _2: Marker2D = $"../SpawnPoints/2"
@onready var _3: Marker2D = $"../SpawnPoints/3"
@onready var _4: Marker2D = $"../SpawnPoints/4"
@onready var game = get_node("/root/Game") as Game

## When the tutorial camera is ready, set the position to the first spawn point
func _ready() -> void:
	self.position = _1.position

## When the player enters the success body, move the camera to the second spawn point
func _on_tutorial_one_success_body_entered(body: Node2D) -> void:
	if body is Player:
		var tween: Tween = create_tween()
		tween.tween_property(self, "position", _2.position, 2.0)
		body.position = _4.position

## When the player enters the second success body, end the tutorial
func _on_tutorial_two_success_body_entered(body: Node2D) -> void:
	if body is Player:
		game.call_deferred("end_tutorial")
