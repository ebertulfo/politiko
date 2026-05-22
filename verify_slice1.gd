# Temporary Slice 1 verifier: loads the real Main scene, reads the status-bar
# labels at tick 0, simulates an Advance Month press, and confirms they update.
# Prints PASS/FAIL per check and ALWAYS quits (never hangs the headless run).
extends SceneTree

var main
var treasury0
var approval0
var tick0
var checks_failed = 0

func _initialize():
	main = load("res://Main.tscn").instantiate()
	get_root().add_child(main)  # parent is in-tree -> _ready fires synchronously

func _process(_delta):
	# Runs after the first frame so _ready/@onready vars are guaranteed set.
	tick0 = main.tick_label.text
	treasury0 = main.treasury_label.text
	approval0 = main.approval_label.text
	var pop0 = main.population_label.text
	print("=== Slice 1 — tick 0 ===")
	print("  ", tick0, " | ", treasury0, " | ", approval0, " | ", pop0)
	print("  engine root population = ", main.engine.get_root()["population"])

	_check("population not hardcoded placeholder", pop0 != "Population: -")

	var approval_f0 = main.engine.get_approval()
	main.advance_button.pressed.emit()
	print("=== after Advance Month ===")
	print("  ", main.tick_label.text, " | ", main.treasury_label.text, " | ", main.approval_label.text)

	_check("tick advanced", main.tick_label.text != tick0)
	_check("engine tick == 1", main.engine.get_tick() == 1)
	_check("treasury changed", main.treasury_label.text != treasury0)
	_check("approval (engine float) changed", main.engine.get_approval() != approval_f0)
	_check("approval label reflects 1-decimal precision", main.approval_label.text.contains("."))

	print("=== Slice 1 ", "OK" if checks_failed == 0 else "FAILED (%d)" % checks_failed, " ===")
	quit(checks_failed)
	return true

func _check(label, ok):
	print("  [", "PASS" if ok else "FAIL", "] ", label)
	if not ok:
		checks_failed += 1
