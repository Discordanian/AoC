class_name DayScene
extends Control

@export var day: int = 0
@export var year: int = 2026
@export var downloadInput: bool = true

@onready var answer1: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/Answer1
@onready var answer2: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/Answer2
@onready var example1: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExAnswer1
@onready var example2: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExAnswer2
@onready var exampleText: TextEdit = $MarginContainer/VBoxContainer/Assignment/Example/ExampleTextEdit
@onready var exampleText2: TextEdit = $MarginContainer/VBoxContainer/Assignment/Example/ExampleTextEdit2
@onready var inputData: CheckBox = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/InputDataReady

var example_path1: String
var example_path2: String


func _ready() -> void:
	example_path1 = AOCUtil.example_path1(year, day)
	example_path2 = AOCUtil.example_path2(year, day)
	var input_path: String = AOCUtil.input_path(year, day)
	if downloadInput:
		AOCUtil.download_file(self, year, day, input_path)
	setup_example()

	# I could/should use an async and a callback but instead I'm going to wait 3 seconds
	# and just see if I have the data file.  Mark me as lazy if you want.
	get_tree().create_timer(3.0).timeout.connect(check_for_input)


func check_for_input() -> void:
	inputData.button_pressed = FileAccess.file_exists(AOCUtil.input_path(year, day))


func setup_example() -> void:
	var content1: String = AOCUtil.string_from_file(example_path1)
	var content2: String = AOCUtil.string_from_file(example_path2)
	exampleText.text = content1
	exampleText2.text = content2


func _on_example_text_edit_text_changed() -> void:
	_save_example(example_path1, exampleText.text)


func _on_example_text_edit_text_changed2() -> void:
	_save_example(example_path2, exampleText2.text)


func _save_example(path: String, content: String) -> void:
	var file: FileAccess = FileAccess.open(path, FileAccess.WRITE)
	if file:
		file.store_string(content)
		file.close()
	else:
		push_error("Error writing to " + path)


func _on_example_pressed() -> void:
	var data1: String = AOCUtil.string_from_file(example_path1)
	var data2: String = AOCUtil.string_from_file(example_path2)
	part1(data1, example1)
	part2(data2, example2)


func _on_input_pressed() -> void:
	var input_path: String = AOCUtil.input_path(year, day)
	if not FileAccess.file_exists(input_path):
		answer1.text = "File not Found"
		answer2.text = "File not Found"
		return
	var data: String = AOCUtil.string_from_file(input_path)
	part1(data, answer1)
	part2(data, answer2)


func _on_main_pressed() -> void:
	get_tree().change_scene_to_file("res://scenes/main_entry.tscn")


func part1(data: String, ans: LineEdit) -> void:
	ans.text = _solver_result("part1", data)


func part2(data: String, ans: LineEdit) -> void:
	ans.text = _solver_result("part2", data)


func _solver_result(part_name: String, data: String) -> String:
	var script: GDScript = load("res://scripts/days/D%02d.gd" % day) as GDScript
	var solver: Object = script.new()
	return str(solver.call(part_name, data))
