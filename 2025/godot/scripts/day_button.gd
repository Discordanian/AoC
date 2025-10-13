extends Button
class_name DayButton

var day: int

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
    pass # Replace with function body.



func setup(d: int) -> void:
    day = d
    # print("Setup called with ", d)
    $".".text = "Day %02d" % d
    # print(text)
