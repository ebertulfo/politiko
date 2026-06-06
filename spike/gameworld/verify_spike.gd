# Headless gate for the gameworld spike. Builds the scene, asserts the terrain
# mesh has real relief, the province-ID map loaded with all regions, and a
# selection resolves to a live engine unit. Exits 0 on success.
# Run: godot --headless --path . --script res://spike/gameworld/verify_spike.gd
extends SceneTree


func _initialize() -> void:
	var root := (load("res://spike/gameworld/Spike3D.tscn") as PackedScene).instantiate()
	get_root().add_child(root)
	await process_frame
	await process_frame

	var fails := 0

	var terrain: MeshInstance3D = root.get_node_or_null("PhilippinesTerrain")
	if terrain == null or terrain.mesh == null:
		push_error("[verify] no terrain mesh"); fails += 1
	else:
		var aabb := terrain.mesh.get_aabb()
		var verts: int = terrain.mesh.surface_get_arrays(0)[Mesh.ARRAY_VERTEX].size()
		print("[verify] verts=%d aabb=%s" % [verts, aabb.size])
		if verts < 10000: push_error("[verify] mesh too sparse"); fails += 1
		if aabb.size.y <= 0.01: push_error("[verify] terrain flat"); fails += 1
		if aabb.size.x < 100.0: push_error("[verify] mesh too small"); fails += 1

	# province-ID map present and covers all 17 regions
	var img: Image = root._province_img
	if img == null:
		push_error("[verify] no province image"); fails += 1
	else:
		var ids := {}
		for y in range(0, img.get_height(), 40):
			for x in range(0, img.get_width(), 40):
				var rid := int(round(img.get_pixel(x, y).r * 255.0))
				if rid > 0: ids[rid] = true
		print("[verify] distinct region ids found: %d" % ids.size())
		if ids.size() < 17: push_error("[verify] expected 17 regions, got %d" % ids.size()); fails += 1

	# selection wires to a live engine unit
	root._set_selection(1)
	var u: Dictionary = root.engine.get_unit(1)
	print("[verify] select id 1 -> '%s' pop=%s" % [u.get("name", "?"), u.get("population", 0)])
	if u.is_empty() or String(u.get("name", "")) == "":
		push_error("[verify] selection did not resolve to a unit"); fails += 1
	if root._selected_id != 1:
		push_error("[verify] selection state not set"); fails += 1

	print("[verify] %s" % ("PASS" if fails == 0 else "FAIL (%d)" % fails))
	quit(fails)
