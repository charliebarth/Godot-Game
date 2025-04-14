extends Map

@onready var game = get_node("/root/Game") as Game
@onready var _5: Marker2D = $"SpawnPoints/5"
@onready var _1: Marker2D = $"SpawnPoints/1"

var players_complete = []

func _on_tutorial_one_success_body_entered(body: Node2D) -> void:
	if body is Player:
		body.position = _5.position


func _on_tutorial_two_success_body_entered(body: Node2D) -> void:
	if body is Player:
		if !body.get_player_id() in players_complete:
			players_complete.append(body.get_player_id())
		
		if len(players_complete) == game.get_number_of_players():
			game.call_deferred("end_tutorial")


func _on_fall_body_entered(body: Node2D) -> void:
	if body is Player:
		body.position = _1.position
