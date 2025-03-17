extends Game
class_name GameGD

var peer = ENetMultiplayerPeer.new()

func _input(event: InputEvent) -> void:
	pass

func host() -> void:
	peer.create_server(25565)
	multiplayer.multiplayer_peer = peer
	
	multiplayer.peer_connected.connect(
		func(pid):
			var num_players = self.get_number_of_players() + 1
			self.create_player(num_players)
			rpc("create_player", num_players)
	)

func join() -> void:
	peer.create_client("localhost", 25565)
	multiplayer.multiplayer_peer = peer

@rpc("any_peer", "call_remote")  
func create_player(num_players: int):
	var num_players_to_create = num_players - self.get_number_of_players() 
	for i in num_players_to_create:
		self.register_player()
