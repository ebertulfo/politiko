# Loads the spike scene in a real (non-headless) window, lets it render a few
# frames, then saves a screenshot to data/render3d.png and quits. For producing
# a visual artifact of the 3D terrain. Run WITHOUT --headless.
extends SceneTree


func _initialize() -> void:
	DisplayServer.window_set_size(Vector2i(820, 1280))
	var scene: PackedScene = load("res://spike/gameworld/Spike3D.tscn")
	var node := scene.instantiate()
	get_root().add_child(node)
	for i in 12:
		await process_frame
	if OS.get_environment("POLITIKO_SELECT") != "":
		node._set_selection(int(OS.get_environment("POLITIKO_SELECT")))
	await create_timer(0.4).timeout
	var img := get_root().get_texture().get_image()
	var name := OS.get_environment("POLITIKO_SHOT")
	var path := "res://spike/gameworld/data/%s" % (name if name != "" else "render3d.png")
	var err := img.save_png(path)
	print("[shoot] saved %s size=%s err=%d" % [path, img.get_size(), err])
	quit(0)
