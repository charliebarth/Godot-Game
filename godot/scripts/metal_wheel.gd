extends CanvasLayer

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("metal_selector"):
		$SelectionWheel.show()
	elif Input.is_action_just_released("metal_selector"):
		var metal = $SelectionWheel.close()
		print("SELECTED: ", metal)
		
		
