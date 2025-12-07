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
    var retval: int = 0
    var splitters: Set = Set.new()
    var lines: PackedStringArray = data.split("\n")
    H = lines.size()
    W = lines[0].length()
    for y: int in range(H):
        for x: int in range(W):
            if lines[y][x] == "S":
                S = Vector2i(x,y)
            if lines[y][x] == "^":
                splitters.add(Vector2i(x,y))

    
    var d: Dictionary = {S : 1} # timelines
    var q: Array[Vector2i] = [S]
    var v: Set = Set.new() # visited
    
    for pos: Vector2i in q:
        if v.contains(pos):
            continue
        v.add(pos)
        
        var w: int = d[pos]
        
        if splitters.contains(pos):
            var left: Vector2i = pos + Vector2i(-1,1)
            var right: Vector2i = pos + Vector2i(1,1)
            q.append(left)
            q.append(right)
            d[left] = d.get(left, 0) + w
            d[right] = d.get(right, 0) + w
        else:
            if pos.y < H - 1:
                var next_pos: Vector2i = pos + Vector2i(0,1)
                q.append(next_pos)
                d[next_pos] = d.get(next_pos, 0) + w
            else:
                retval += w
    
            
            
    
    return str(retval)
