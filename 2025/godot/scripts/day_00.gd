extends Control

@export var day: int = 0
@onready var answer1: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/Answer1
@onready var answer2: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/Answer2
@onready var example1: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExAnswer1
@onready var example2: LineEdit = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExAnswer2
@onready var exampleText: TextEdit = $MarginContainer/VBoxContainer/Assignment/Example/ExampleTextEdit
@onready var exampleData: CheckBox = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/ExampleDataReady
@onready var inputData: CheckBox = $MarginContainer/VBoxContainer/Assignment/VBoxContainer/InputDataReady

var example_path: String = "user://2025ex00.txt"

func setup_example() -> void:
    var content: String = ""
    if FileAccess.file_exists(example_path):
        var file: FileAccess = FileAccess.open(example_path, FileAccess.READ)
        if file:
            content = file.get_as_text()
            file.close()
        else:
            print("Error reading data from " + example_path)
    else:
        print("Example input file not already present") 
    exampleText.text = content  

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
    setup_example()
        
    
func part1() -> void:
    pass


func _on_example_text_edit_text_changed() -> void:
    exampleData.button_pressed = exampleText.text.length() != 0
    var file: FileAccess = FileAccess.open(example_path, FileAccess.WRITE)
    if file:
        file.store_string(exampleText.text)
        file.close()
    else:
        print("Error writing to " + example_path)
    
