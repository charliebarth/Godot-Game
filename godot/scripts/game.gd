extends Game
class_name GameGD

var peer = ENetMultiplayerPeer.new()
var num_peers = 0
var serialized_data: Array[Dictionary] = []
var ready_peers: Array[int] = []

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

func add_serialization(data: Dictionary):
	serialized_data.append(data)
	
	if len(serialized_data) == 2:
		rpc("receive_server_data", serialized_data)
		serialized_data.clear()

func join() -> void:
	peer.create_client("localhost", 25565)
	multiplayer.multiplayer_peer = peer

@rpc("any_peer", "call_remote")
func receive_server_data(data: Array[Dictionary]):
	for player_data in data:
		var player_id = player_data["player_id"] as int
		self.update_player_data(player_data, player_id)

@rpc("any_peer", "call_remote")
func create_player(num_players: int):
	var num_players_to_create = num_players - self.get_number_of_players()
	for i in num_players_to_create:
		self.register_player(-1)

@rpc("any_peer", "call_remote")
func set_peer_number_godot(peer_num: int):
	self.set_peer_number(peer_num)

@rpc("any_peer", "call_remote")
func start():
	self.start_game()

@rpc("any_peer", "call_remote")
func receive_input(input_data: Dictionary):
	self.handle_input(int(input_data["player_id"]),
		input_data["button_name"],
		input_data["is_pressed"],
		input_data["is_released"],
		float(input_data["action_strength"]))
		
	if multiplayer.is_server():
		for pid in multiplayer.get_peers():
			if pid != multiplayer.get_unique_id():
				rpc_id(pid, "receive_input", input_data)
