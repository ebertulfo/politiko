# Slice 1 — National dashboard. Instantiates the Rust engine, shows the national
# readout in a top status bar, and advances the clock one month per button press.
# All values are pulled live from PolitikoEngine; nothing here is hardcoded.
extends Control

var engine: PolitikoEngine

@onready var tick_label: Label = %TickLabel
@onready var treasury_label: Label = %TreasuryLabel
@onready var approval_label: Label = %ApprovalLabel
@onready var population_label: Label = %PopulationLabel
@onready var advance_button: Button = %AdvanceButton


func _ready() -> void:
	engine = PolitikoEngine.new()
	advance_button.pressed.connect(_on_advance_pressed)
	_refresh()


func _on_advance_pressed() -> void:
	engine.advance_tick()
	_refresh()


func _refresh() -> void:
	var root: Dictionary = engine.get_root()
	tick_label.text = "Month %d" % engine.get_tick()
	treasury_label.text = "Treasury: %s" % _format_peso(engine.get_treasury())
	approval_label.text = "Approval: %.1f%%" % (engine.get_approval() * 100.0)
	population_label.text = "Population: %s" % _format_count(root["population"])


# ₱ with a magnitude suffix (B/M) so a ₱500,000,000,000 treasury reads as "₱500.0B".
func _format_peso(value: float) -> String:
	var sign := "-" if value < 0.0 else ""
	var v := absf(value)
	if v >= 1.0e9:
		return "%s₱%.1fB" % [sign, v / 1.0e9]
	if v >= 1.0e6:
		return "%s₱%.1fM" % [sign, v / 1.0e6]
	return "%s₱%.0f" % [sign, v]


# Whole population count with a magnitude suffix (109,000,000 → "109.0M").
func _format_count(value: int) -> String:
	if value >= 1_000_000:
		return "%.1fM" % (value / 1.0e6)
	if value >= 1_000:
		return "%.1fK" % (value / 1.0e3)
	return str(value)
