extends Control

@export var day: int = 0
@export var year: int = 2025
@onready var answer1: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/Answer1
@onready var answer2: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/Answer2
@onready var example1: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExAnswer1
@onready var example2: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExAnswer2
@onready var exampleText: TextEdit = $MarginContainer/VBoxContainer/Assignment/Example/ExampleTextEdit
@onready var exampleData: CheckBox = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExampleDataReady
@onready var inputData: CheckBox = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/InputDataReady

var example_path: String

func check_for_input() -> void:
    inputData.button_pressed = FileAccess.file_exists(AOCUtil.input_path(2024,10))

func setup_example() -> void:
    var content: String =  AOCUtil.string_from_file(example_path)
    exampleText.text = content
    exampleData.button_pressed = exampleText.text.length() != 0

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
    example_path = AOCUtil.example_path(year,day)
    print(example_path)
    var input_path: String = AOCUtil.input_path(2024,10)
    AOCUtil.download_file(self, 2024,10,input_path)
    setup_example()
    check_for_input()
        
    
func part1(data: String, ans: LineEdit) -> void:
    var answer: String
    answer = str(AOCUtil.string_to_lines(data).size())
    ans.text = answer
    
    
func part2(data: String, ans: LineEdit) -> void:
    var answer: String
    answer = str(AOCUtil.dimensions_of_2d_array(AOCUtil.string_to_2d_char_array(data)))
    ans.text = answer


func _on_example_text_edit_text_changed() -> void:
    exampleData.button_pressed = exampleText.text.length() != 0
    var file: FileAccess = FileAccess.open(example_path, FileAccess.WRITE)
    if file:
        file.store_string(exampleText.text)
        file.close()
    else:
        print("Error writing to " + example_path)
    


func _on_example_pressed() -> void:
    var data: String = AOCUtil.string_from_file(AOCUtil.example_path(year, day))
    part1(data, example1)
    part2(data, example2)
    
func _on_input_pressed() -> void:
    var data: String = AOCUtil.string_from_file(AOCUtil.input_path(2024, 10))
    part1(data, answer1)
    part2(data, answer2)
