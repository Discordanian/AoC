extends Control

@onready var grid: GridContainer = $MarginContainer/VBoxContainer/ButtonGrid

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
    for i: int in range(25):
        # print("Button Loop ", i)
        var b: DayButton = DayButton.new()
        b.disabled = true
        b.setup(i+1)
        grid.add_child(b)
        


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
    pass
