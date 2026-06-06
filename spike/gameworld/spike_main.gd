# THROWAWAY SPIKE — Philippines 3D gameworld map.
# Displaced DEM mesh + draped relief texture + a province-ID overlay: click a
# region to select it (border highlight) and read its LIVE stats from the Rust
# engine (PolitikoEngine). Left-click = select, right-drag = orbit, wheel = zoom.
extends Node3D

const DATA := "res://spike/gameworld/data/"
const MAP_WIDTH := 220.0
const TARGET_GRID := 1300
const VERTICAL_EXAGGERATION := 2.5   # lower = shorter mountains (1.5 flat … 5 dramatic)
const HEIGHT_CURVE_K := 8.0          # 0 = linear; >0 = log-compress the tall peaks

const SHADER := """
shader_type spatial;
render_mode unshaded, cull_disabled;
uniform sampler2D relief_tex : source_color, filter_linear;
uniform sampler2D province_tex : filter_nearest;
uniform float selected_id = 0.0;
uniform vec2 texel = vec2(0.0002);
void fragment() {
	vec3 base = texture(relief_tex, UV).rgb;
	float id = texture(province_tex, UV).r * 255.0;
	float idu = texture(province_tex, UV + vec2(0.0, -texel.y)).r * 255.0;
	float idd = texture(province_tex, UV + vec2(0.0,  texel.y)).r * 255.0;
	float idl = texture(province_tex, UV + vec2(-texel.x, 0.0)).r * 255.0;
	float idr = texture(province_tex, UV + vec2( texel.x, 0.0)).r * 255.0;
	float diff = step(0.5, abs(id-idu)) + step(0.5, abs(id-idd))
			   + step(0.5, abs(id-idl)) + step(0.5, abs(id-idr));
	float is_border = step(0.5, diff) * step(0.5, id);   // land-side edges only
	vec3 col = base;
	if (selected_id > 0.5 && abs(id - selected_id) < 0.5) {
		col = mix(col, vec3(1.0, 0.85, 0.25), 0.40);
	}
	col = mix(col, vec3(0.08, 0.06, 0.10), is_border * 0.85);
	ALBEDO = col;
}
"""

var _pivot := Vector3.ZERO
var _yaw := PI
var _pitch := -1.2
var _dist := 300.0
var _depth := 350.0
var _camera: Camera3D
var _terrain_mat: ShaderMaterial
var _province_img: Image

var engine: PolitikoEngine
var _selected_id := 0
var _info_label: Label
var _nation_btn: Button


func _ready() -> void:
	engine = PolitikoEngine.new()
	var stats := _build_terrain()
	_setup_environment()
	_build_ui()
	_refresh_nation()
	print("[spike] terrain built: %d verts, grid %dx%d" % [stats.verts, stats.cols, stats.rows])


func _build_terrain() -> Dictionary:
	var meta: Dictionary = JSON.parse_string(FileAccess.get_file_as_string(DATA + "meta.json"))
	var img := Image.load_from_file(DATA + "heightmap_rg.png")
	var w := img.get_width()
	var h := img.get_height()

	var exag := VERTICAL_EXAGGERATION
	var curve_k := HEIGHT_CURVE_K
	if OS.get_environment("POLITIKO_EXAG") != "":
		exag = float(OS.get_environment("POLITIKO_EXAG"))
	if OS.get_environment("POLITIKO_CURVE") != "":
		curve_k = float(OS.get_environment("POLITIKO_CURVE"))

	var stride: int = maxi(1, int(ceil(float(maxi(w, h)) / TARGET_GRID)))
	var xs := range(0, w, stride)
	var ys := range(0, h, stride)
	var cols := xs.size()
	var rows := ys.size()
	_depth = MAP_WIDTH * (float(h) / float(w))
	var max_m: float = meta.get("max_elevation_m", 3000.0)
	var meters_per_unit: float = (meta.get("approx_m_per_px_x", 300.0) * w) / MAP_WIDTH
	var y_scale := (max_m / meters_per_unit) * exag

	var verts := PackedVector3Array()
	var uvs := PackedVector2Array()
	verts.resize(cols * rows)
	uvs.resize(cols * rows)
	for iy in rows:
		var py: int = ys[iy]
		var vz := (float(iy) / float(rows - 1) - 0.5) * _depth
		for ix in cols:
			var c := img.get_pixel(xs[ix], py)
			var n := float(int(round(c.r * 255.0)) * 256 + int(round(c.g * 255.0))) / 65535.0
			var hn := n if curve_k <= 0.0 else log(1.0 + curve_k * n) / log(1.0 + curve_k)
			var k := iy * cols + ix
			verts[k] = Vector3((float(ix) / float(cols - 1) - 0.5) * MAP_WIDTH, hn * y_scale, vz)
			uvs[k] = Vector2(float(xs[ix]) / w, float(py) / h)

	var indices := PackedInt32Array()
	indices.resize((cols - 1) * (rows - 1) * 6)
	var t := 0
	for iy in rows - 1:
		for ix in cols - 1:
			var a := iy * cols + ix
			indices[t] = a; indices[t + 1] = a + cols; indices[t + 2] = a + 1
			indices[t + 3] = a + 1; indices[t + 4] = a + cols; indices[t + 5] = a + cols + 1
			t += 6

	var arrays := []
	arrays.resize(Mesh.ARRAY_MAX)
	arrays[Mesh.ARRAY_VERTEX] = verts
	arrays[Mesh.ARRAY_TEX_UV] = uvs
	arrays[Mesh.ARRAY_INDEX] = indices
	var mesh := ArrayMesh.new()
	mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arrays)

	var shader := Shader.new()
	shader.code = SHADER
	_terrain_mat = ShaderMaterial.new()
	_terrain_mat.shader = shader
	_terrain_mat.set_shader_parameter("relief_tex",
		ImageTexture.create_from_image(Image.load_from_file(DATA + "relief.png")))
	var prov_tex := ImageTexture.create_from_image(Image.load_from_file(DATA + "provinces.png"))
	_terrain_mat.set_shader_parameter("province_tex", prov_tex)
	_terrain_mat.set_shader_parameter("texel", Vector2(1.0 / w, 1.0 / h))

	var mi := MeshInstance3D.new()
	mi.name = "PhilippinesTerrain"
	mi.mesh = mesh
	mi.material_override = _terrain_mat
	add_child(mi)

	_province_img = Image.load_from_file(DATA + "provinces.png")   # CPU copy for click-picking
	_dist = _depth * 0.92
	return {"verts": cols * rows, "cols": cols, "rows": rows}


func _setup_environment() -> void:
	var env := Environment.new()
	env.background_mode = Environment.BG_COLOR
	env.background_color = Color(0.07, 0.16, 0.27)
	var we := WorldEnvironment.new()
	we.environment = env
	add_child(we)
	if OS.get_environment("POLITIKO_PITCH") != "":
		_pitch = float(OS.get_environment("POLITIKO_PITCH"))
	_camera = Camera3D.new()
	_camera.far = 6000.0
	add_child(_camera)
	_update_camera()


# ---------------------------------------------------------------- selection ---

func _select_at(mouse_pos: Vector2) -> void:
	if _camera == null or _province_img == null:
		return
	var from := _camera.project_ray_origin(mouse_pos)
	var dir := _camera.project_ray_normal(mouse_pos)
	if absf(dir.y) < 1e-5:
		return
	var dist := -from.y / dir.y          # intersect the y=0 sea plane
	if dist < 0.0:
		return
	var hit := from + dir * dist
	var u := hit.x / MAP_WIDTH + 0.5
	var v := hit.z / _depth + 0.5
	if u < 0.0 or u > 1.0 or v < 0.0 or v > 1.0:
		return
	var px := clampi(int(u * _province_img.get_width()), 0, _province_img.get_width() - 1)
	var py := clampi(int(v * _province_img.get_height()), 0, _province_img.get_height() - 1)
	_set_selection(int(round(_province_img.get_pixel(px, py).r * 255.0)))


func _set_selection(id: int) -> void:
	_selected_id = id
	_terrain_mat.set_shader_parameter("selected_id", float(id))
	_refresh_info()


func _refresh_info() -> void:
	if _selected_id <= 0:
		_info_label.text = "Click a region to inspect it."
		return
	var u: Dictionary = engine.get_unit(_selected_id)
	if u.is_empty():
		_info_label.text = "Click a region to inspect it."
		return
	var inds := PackedStringArray()
	for ind in u.get("industries", []):
		inds.append("%s L%d" % [ind["kind"], ind["level"]])
	_info_label.text = "\n".join([
		"[ %s ]" % u["name"],
		"Population   %s" % _fmt_count(u["population"]),
		"Satisfaction %.0f%%" % (float(u["satisfaction"]) * 100.0),
		"Revenue/mo   %s" % _fmt_peso(u["revenue_last_tick"]),
		"Poverty      %.0f%%" % (float(u["poverty"]) * 100.0),
		"Employment   %.0f%%" % (float(u["employment_rate"]) * 100.0),
		"Daily wage   %s" % _fmt_peso(u["daily_wage"]),
		"Education    %.0f%%" % (float(u["education_level"]) * 100.0),
		"Transport    L%d" % int(u["transport_level"]),
		"Elevation    %.0f m  (rugged %.2f)" % [float(u["avg_elevation"]), float(u["terrain_ruggedness"])],
		"Coastal      %s" % ("yes" if u["coastal"] else "no"),
		"Industries   %s" % (", ".join(inds) if inds.size() > 0 else "—"),
	])


func _refresh_nation() -> void:
	var root: Dictionary = engine.get_root()
	_nation_btn.text = "  Month %d     Treasury %s     Approval %.1f%%     Pop %s         Advance Month ▶" % [
		engine.get_tick(), _fmt_peso(engine.get_treasury()),
		engine.get_approval() * 100.0, _fmt_count(root["population"])]


func _on_advance() -> void:
	engine.advance_tick()
	_refresh_nation()
	_refresh_info()


# ---------------------------------------------------------------------- ui ---

func _build_ui() -> void:
	var layer := CanvasLayer.new()
	add_child(layer)

	_nation_btn = Button.new()
	_nation_btn.alignment = HORIZONTAL_ALIGNMENT_LEFT
	_nation_btn.add_theme_color_override("font_color", Color(0.92, 0.95, 1.0))
	_nation_btn.set_anchors_preset(Control.PRESET_TOP_WIDE)
	_nation_btn.custom_minimum_size = Vector2(0, 36)
	var nb := StyleBoxFlat.new()
	nb.bg_color = Color(0.04, 0.07, 0.12, 0.92)
	_nation_btn.add_theme_stylebox_override("normal", nb)
	_nation_btn.add_theme_stylebox_override("hover", nb)
	_nation_btn.add_theme_stylebox_override("pressed", nb)
	_nation_btn.pressed.connect(_on_advance)
	layer.add_child(_nation_btn)

	var panel := PanelContainer.new()
	panel.set_anchors_preset(Control.PRESET_TOP_LEFT)
	panel.position = Vector2(12, 46)
	panel.custom_minimum_size = Vector2(330, 0)
	var sb := StyleBoxFlat.new()
	sb.bg_color = Color(0.05, 0.08, 0.13, 0.86)
	sb.set_corner_radius_all(8)
	sb.set_content_margin_all(12)
	panel.add_theme_stylebox_override("panel", sb)
	layer.add_child(panel)
	_info_label = Label.new()
	_info_label.add_theme_font_size_override("font_size", 14)
	_info_label.add_theme_color_override("font_color", Color(0.88, 0.92, 0.98))
	_info_label.text = "Click a region to inspect it."
	panel.add_child(_info_label)


# ----------------------------------------------------------------- camera ---

func _update_camera() -> void:
	if _camera == null:
		return
	var dir := Vector3(cos(_pitch) * sin(_yaw), sin(_pitch), cos(_pitch) * cos(_yaw))
	_camera.position = _pivot - dir * _dist
	_camera.look_at(_pivot, Vector3.UP)


func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed:
		match event.button_index:
			MOUSE_BUTTON_LEFT:
				_select_at(event.position)
			MOUSE_BUTTON_WHEEL_UP:
				_dist = maxf(_dist * 0.9, 20.0); _update_camera()
			MOUSE_BUTTON_WHEEL_DOWN:
				_dist = minf(_dist * 1.1, 2000.0); _update_camera()
	elif event is InputEventMouseMotion and (event.button_mask & MOUSE_BUTTON_MASK_RIGHT):
		_yaw -= event.relative.x * 0.005
		_pitch = clampf(_pitch - event.relative.y * 0.005, -1.55, -0.12)
		_update_camera()


# ------------------------------------------------------------- formatting ---

func _fmt_count(v: int) -> String:
	if v >= 1_000_000:
		return "%.1fM" % (v / 1.0e6)
	if v >= 1_000:
		return "%.1fK" % (v / 1.0e3)
	return str(v)


func _fmt_peso(value: float) -> String:
	var s := "-" if value < 0.0 else ""
	var v := absf(value)
	if v >= 1.0e9:
		return "%s₱%.1fB" % [s, v / 1.0e9]
	if v >= 1.0e6:
		return "%s₱%.1fM" % [s, v / 1.0e6]
	if v >= 1.0e3:
		return "%s₱%.1fK" % [s, v / 1.0e3]
	return "%s₱%.0f" % [s, v]
