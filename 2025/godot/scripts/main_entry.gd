extends Control

@onready var grid: GridContainer = $MarginContainer/VBoxContainer/ButtonGrid

func is_day_scene_missing(day: int) -> bool:
	var scene_path: String = "res://scenes/day_%02d.tscn" % day
	if FileAccess.file_exists(scene_path):
		print("I see file: ", scene_path)
	else:
		print("I do NOT see file: ", scene_path)	
	return not ResourceLoader.exists(scene_path)


func _on_day_button_pressed(day: int) -> void:
	var scene_path: String = "res://scenes/day_%02d.tscn" % day
	
	if not is_day_scene_missing(day):
		print("Loading scene for day %02d: %s" % [day,scene_path])
		
		var scene_resource: PackedScene = load(scene_path)
		if scene_resource:
			get_tree().change_scene_to_packed(scene_resource)
		else:
			push_error("Error loading scene!")
	else:
		push_error("Told to load non existant scene")				

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	for i: int in range(25):
		var b: Button = Button.new()
		b.text = "Day %02d" % i
		# var b: DayButton = DayButton.new()
		b.disabled = is_day_scene_missing(i)
		# b.setup(i+1)
		grid.add_child(b)
		b.pressed.connect(_on_day_button_pressed.bind(i))
		
