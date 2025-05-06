extends Game
class_name GameGD

var peer = ENetMultiplayerPeer.new()
var num_peers = 0
var serialized_data: Array[Dictionary] = []
var ready_peers: Array[int] = []

## Creates a server for online multiplayer
func host() -> void:
	peer.create_server(25565)
	multiplayer.multiplayer_peer = peer
	
	multiplayer.peer_connected.connect(
		func(pid):
			var num_players = self.get_number_of_players() + 1
			rpc_id(pid, "set_peer_number_godot", num_peers)
			self.create_player(num_players)
			rpc("create_player", num_players)
			num_peers += 1
	)

## Called when a client is ready to start the game
@rpc("any_peer", "call_remote")
func ready(id: int):
	if id in ready_peers:
		var index = ready_peers.find(id)
		ready_peers.remove_at(index)
	else:
		ready_peers.append(id)
		
	if len(ready_peers) == num_peers:
		self.start()
		rpc("start")

## Adds data to the serialized data array
## When all the collected data is present, it will be sent to all the clients via RPC
func add_serialization(data: Dictionary):
	serialized_data.append(data)
	
	if len(serialized_data) == 2:
		rpc("receive_server_data", serialized_data)
		serialized_data.clear()

## Joins a server for online multiplayer
func join() -> void:
	peer.create_client("100.65.218.78", 25565)
	multiplayer.multiplayer_peer = peer

## Receives the serialized data from the server
## The data is then deserialized and used to update the player's data
@rpc("any_peer", "call_remote")
func receive_server_data(data: Array[Dictionary]):
	for player_data in data:
		var player_id = player_data["player_id"] as int
		self.update_player_data(player_data, player_id)

## Creates a player for the game
## This is called when a client connects to the server
@rpc("any_peer", "call_remote")
func create_player(num_players: int):
	var num_players_to_create = num_players - self.get_number_of_players()
	for i in num_players_to_create:
		self.register_player(-1)

## Sets the peer number for the local player
@rpc("any_peer", "call_remote")
func set_peer_number_godot(peer_num: int):
	self.set_peer_number(peer_num)

## Starts the game
@rpc("any_peer", "call_remote")
func start():
	self.start_game()

## Receives input from the client
## The input is then handled by the game
@rpc("any_peer", "call_remote")
func receive_input(input_data: Dictionary):
	self.handle_input(int(input_data["player_id"]),
		input_data["button_name"],
		input_data["is_pressed"],
		input_data["is_released"],
		float(input_data["action_strength"]))
		
	if multiplayer.is_server():
		rpc("receive_input", input_data)

## Receives movement input from the client
## The movement input is then handled by the game
@rpc("any_peer", "call_remote")
func receive_movement(player_id: int, left: float, right: float, line_selector_position: Vector2, trigger_left: float, trigger_right: float):
	self.handle_movement(player_id, left, right, line_selector_position, trigger_left, trigger_right)
		
	if multiplayer.is_server():
		rpc("receive_movement", player_id, left, right, line_selector_position, trigger_left, trigger_right)
		

## Removes a player when they die and notifies all clients of the player death
@rpc("any_peer", "call_remote")
func player_death(player_id: int, elims: int):
	self.remove_player(player_id, elims)
	
	if multiplayer.is_server():
		rpc("player_death", player_id, elims)
