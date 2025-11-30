extends Control

@export var day: int = 2
@export var year: int = 2025
@export var downloadInput: bool = true

#region Boilerplate
@onready var answer1: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/Answer1
@onready var answer2: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/Answer2
@onready var example1: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExAnswer1
@onready var example2: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExAnswer2
@onready var exampleText: TextEdit = $MarginContainer/VBoxContainer/Assignment/Example/ExampleTextEdit
@onready var exampleText2: TextEdit = $MarginContainer/VBoxContainer/Assignment/Example/ExampleTextEdit2
@onready var inputData: CheckBox = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/InputDataReady
@onready var exampleButton: Button = %ExampleRunButton
@onready var inputButton: Button = %InputRunButton


var example_path1: String
var example_path2: String

func check_for_input() -> void:
    inputData.button_pressed = FileAccess.file_exists(AOCUtil.input_path(year,day))
    inputButton.disabled = !inputData.button_pressed
    # print("check_for_input()")

func setup_example() -> void:
    var content1: String =  AOCUtil.string_from_file(example_path1)
    var content2: String =  AOCUtil.string_from_file(example_path2)
    exampleText.text = content1
    exampleText2.text = content2
    # exampleData.button_pressed = exampleText.text.length() != 0 && exampleText2.text.length() != 0
    exampleButton.disabled = !(exampleText.text.length() != 0 && exampleText2.text.length() != 0)

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
    example_path1 = AOCUtil.example_path1(year,day)
    example_path2 = AOCUtil.example_path2(year,day)
    var input_path: String = AOCUtil.input_path(year,day)
    if downloadInput:
        AOCUtil.download_file(self, year,day,input_path)
    setup_example()
    
    # I could/should use an async and a callback but instead I'm going to wait 3 seconds
    # and just see if I have the data file.  Mark me as lazy if you want.
    get_tree().create_timer(3.0).timeout.connect(check_for_input)


func _on_example_text_edit_text_changed() -> void:
    # exampleData.button_pressed = exampleText.text.length() != 0
    exampleButton.disabled = exampleText.text.length() == 0 || exampleText.text.length() == 0
    var file: FileAccess = FileAccess.open(example_path1, FileAccess.WRITE)
    if file:
        file.store_string(exampleText.text)
        file.close()
    else:
        push_error("Error writing to " + example_path1)
    
func _on_example_text_edit_text_changed2() -> void:
    # exampleData.button_pressed = exampleText.text.length() != 0
    exampleButton.disabled = exampleText.text.length() == 0 || exampleText.text.length() == 0
    var file: FileAccess = FileAccess.open(example_path2, FileAccess.WRITE)
    if file:
        file.store_string(exampleText2.text)
        file.close()
    else:
        push_error("Error writing to " + example_path2)

func _on_example_pressed() -> void:
    # var data1: String = AOCUtil.string_from_file(AOCUtil.example_path1(year, day))
    # var data2: String = AOCUtil.string_from_file(AOCUtil.example_path2(year, day))
    var data1: String = AOCUtil.string_from_file(example_path1)
    var data2: String = AOCUtil.string_from_file(example_path2)
    part1(data1, example1)
    part2(data2, example2)
    
func _on_input_pressed() -> void:
    var data: String = AOCUtil.string_from_file(AOCUtil.input_path(2024, 10))
    part1(data, answer1)
    part2(data, answer2)


func _on_main_pressed() -> void:
    get_tree().change_scene_to_file("res://scenes/main_entry.tscn") 
#endregion    
    
    
func part1(data: String, ans: LineEdit) -> void:
    ans.text = D02.new().part1(data)
      
func part2(data: String, ans: LineEdit) -> void:
    ans.text = D02.new().part2(data)   
