extends Map

@onready var game = get_node("/root/Game") as Game
@onready var _5: Marker2D = $"SpawnPoints/5"
@onready var _1: Marker2D = $"SpawnPoints/1"

## A list of players that have completed the tutorial
var players_complete = []

## When the player enters the first success body, move the player to the fifth spawn point
func _on_tutorial_one_success_body_entered(body: Node2D) -> void:
	if body is Player:
		body.position = _5.position

## When the player enters the second success body, add the player to the list of players that have completed the tutorial
func _on_tutorial_two_success_body_entered(body: Node2D) -> void:
	if body is Player:
		if !body.get_player_id() in players_complete:
			players_complete.append(body.get_player_id())
		
		if len(players_complete) == game.get_number_of_players():
			game.call_deferred("end_tutorial")

## When the player falls off the map, move the player to the first spawn point
func _on_fall_body_entered(body: Node2D) -> void:
	if body is Player:
		body.position = _1.position
