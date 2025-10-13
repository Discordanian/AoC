extends Control

@export var day: int = 0
@export var year: int = 2025
@export var downloadInput: bool = true
@onready var answer1: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/Answer1
@onready var answer2: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/Answer2
@onready var example1: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExAnswer1
@onready var example2: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExAnswer2
@onready var exampleText: TextEdit = $MarginContainer/VBoxContainer/Assignment/Example/ExampleTextEdit
@onready var exampleData: CheckBox = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExampleDataReady
@onready var inputData: CheckBox = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/InputDataReady
@onready var exampleButton: Button = $MarginContainer/VBoxContainer/Header/Example
@onready var inputButton: Button = $MarginContainer/VBoxContainer/Header/Input


var example_path: String

func check_for_input() -> void:
    inputData.button_pressed = FileAccess.file_exists(AOCUtil.input_path(year,day))
    inputButton.disabled = !inputData.button_pressed
    # print("check_for_input()")

func setup_example() -> void:
    var content: String =  AOCUtil.string_from_file(example_path)
    exampleText.text = content
    exampleData.button_pressed = exampleText.text.length() != 0
    exampleButton.disabled = !exampleData.button_pressed

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
    example_path = AOCUtil.example_path(year,day)
    print(example_path)
    var input_path: String = AOCUtil.input_path(year,day)
    if downloadInput:
        AOCUtil.download_file(self, year,day,input_path)
    setup_example()
    
    # I could/should use an async and a callback but instead I'm going to wait 3 seconds
    # and just see if I have the data file.  Mark me as lazy if you want.
    get_tree().create_timer(3.0).timeout.connect(check_for_input)
        
    
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
    exampleButton.disabled = !exampleData.button_pressed
    var file: FileAccess = FileAccess.open(example_path, FileAccess.WRITE)
    if file:
        file.store_string(exampleText.text)
        file.close()
    else:
        push_error("Error writing to " + example_path)
    

func _on_example_pressed() -> void:
    var data: String = AOCUtil.string_from_file(AOCUtil.example_path(year, day))
    part1(data, example1)
    part2(data, example2)
    
func _on_input_pressed() -> void:
    var data: String = AOCUtil.string_from_file(AOCUtil.input_path(2024, 10))
    part1(data, answer1)
    part2(data, answer2)


func _on_main_pressed() -> void:
    get_tree().change_scene_to_file("res://scenes/main_entry.tscn") 
