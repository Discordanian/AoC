class_name D07 extends RefCounted

var S: Vector2i = Vector2i.ZERO
var H: int = 0
var W: int = 0

func part1(data: String) -> String:

    var splitters: Set = Set.new()
    var splitters_hit: Set = Set.new()
    var lines: PackedStringArray = data.split("\n")
    H = lines.size()
    W = lines[0].length()
    for y: int in range(H):
        for x: int in range(W):
            if lines[y][x] == "S":
                S = Vector2i(x,y)
            if lines[y][x] == "^":
                splitters.add(Vector2i(x,y))
    # print(splitters)
    # print(S)
    var beams: Set = Set.new()
    beams.add(S)
    for h: int in range(H):
        var next_beams: Set = Set.new()
        for beam: Vector2i in beams:
            var down_one: Vector2i = beam + Vector2i(0,1)
            if splitters.contains(down_one):
                splitters_hit.add(down_one)
                next_beams.add(beam + Vector2i(-1,1))
                next_beams.add(beam + Vector2i(1,1))
            else:
                next_beams.add(down_one)
        beams = next_beams
            
            
    
    return str(splitters_hit.size())
    
func part2(data: String) -> String:
    return data
