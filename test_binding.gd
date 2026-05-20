# Headless smoke test for the GDExtension binding.
# Run: godot --headless --script res://test_binding.gd
extends SceneTree

func _initialize():
	print("=== Politiko binding smoke test ===")
	var engine = PolitikoEngine.new()
	print("version            : ", engine.version())
	print("tick               : ", engine.get_tick())
	print("treasury (PHP)     : ", engine.get_treasury())
	print("approval           : ", engine.get_approval())
	print("operational units  : ", engine.operational_unit_count())

	var root = engine.get_root()
	print("national name      : ", root["name"])
	print("national population: ", root["population"])
	print("national revenue   : ", root["revenue_last_tick"])

	var ncr = engine.get_unit(1)
	print("unit 1 name        : ", ncr["name"], "  coastal=", ncr["coastal"], "  rugged=", ncr["terrain_ruggedness"])

	# Topology check: rugged inland CAR (2) must cost more than flat coastal NCR (1).
	print("transport cost NCR : ", engine.transport_cost(1))
	print("transport cost CAR : ", engine.transport_cost(2))

	var out = engine.advance_tick()
	print("after 1 tick       : ", out)
	print("=== OK ===")
	quit()
