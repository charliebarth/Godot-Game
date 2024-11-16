extends Node2D


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var children = self.get_children()

	for child in children:
		if child is TileMapLayer:
			var layer = 2
			for i in range(4):
				var duplicate: TileMapLayer = child.duplicate()

				duplicate.visibility_layer = (1 << layer | 1 << (layer - 1))
				duplicate.light_mask = (1 << layer | 1 << (layer - 1))

				# Optional: Give each duplicate a unique name for easier debugging
				duplicate.name = child.name + "_duplicate_" + str(i + 1)
				
				self.add_child(duplicate)
				
				layer += 2
