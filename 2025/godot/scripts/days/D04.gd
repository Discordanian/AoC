class_name D04 extends RefCounted

var H:int = 0
var W:int = 0

func neighbor_count(s: Set, pos: Vector2i) -> int:
	var retval: int = 0
	for delta: Vector2i in AOCUtil.ALL_NEIGHBORS:
		var n: Vector2i = delta + pos
		if s.contains(n):
			retval += 1
	
	return retval

func part1(data: String) -> String:
	var retval: int = 0
	var lines: PackedStringArray = data.split("\n")
	var rolls: Set = Set.new()
	H = lines.size()
	W = lines[0].length()
	for y: int in range(H):
		for x: int in range(W):
			if lines[y][x] == "@":
				rolls.add(Vector2i(x,y))
	
	for y: int in range(H):
		for x: int in range(W):
			var pos: Vector2i = Vector2i(x,y)
			if rolls.contains(pos) and neighbor_count(rolls, pos) < 4:
				retval += 1
	
	return str(retval)
	
func part2(data: String) -> String:
	var retval: int = 0
	var lines: PackedStringArray = data.split("\n")
	var rolls: Set = Set.new()
	H = lines.size()
	W = lines[0].length()
	for y: int in range(H):
		for x: int in range(W):
			if lines[y][x] == "@":
				rolls.add(Vector2i(x,y))
	
	var prev_retval: int = retval - 1
	
	while prev_retval < retval:
		prev_retval = retval
		for y: int in range(H):
			for x: int in range(W):
				var pos: Vector2i = Vector2i(x,y)
				if rolls.contains(pos) and neighbor_count(rolls, pos) < 4:
					rolls.remove(pos)
					retval += 1
		
	
	return str(retval)
