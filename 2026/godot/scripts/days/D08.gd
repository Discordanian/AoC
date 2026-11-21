class_name D08 extends RefCounted

var a1: int = 0
var a2: int = 0

func both(data: String) -> void:
	var P: Array[Vector3i] = []
	var lines: PackedStringArray = data.split("\n")
	
	for line: String in lines:
		var s: String = line.strip_edges()
		if s.is_empty():
			continue
		var numbers: PackedStringArray = s.split(",", false)
		assert(numbers.size() == 3)
		
		P.append(Vector3i(numbers[0].to_int(), numbers[1].to_int(), numbers[2].to_int()))
	
	var n: int = P.size()
	assert(n>0)
	
	var D: Array[Array] = []
	
	for i: int in range(n):
		for j: int in range(n):
			if i > j:
				var dx: int = P[i].x - P[j].x
				var dy: int = P[i].y - P[j].y
				var dz: int = P[i].z - P[j].z
				var dist: int = dx * dx + dy * dy + dz * dz
				var edge: Array[int] = [dist, i, j]
				D.append(edge)
	
	D.sort()
	
	var uf: UnionFind = UnionFind.new(n)
	var connections: int = 0
	
	for t: int in range(D.size()):
		var edge: Array[int] = D[t]
		var i: int = edge[1]
		var j: int = edge[2]
		
		if t == 1000:
			print("In t 1000")
			var sizes: Dictionary = {}
			for x: int in range(n):
				var r: int = uf.find(x)
				var prev: int = int(sizes.get(r,0))
				sizes[r] = prev + 1
			
			var s_arr: Array[int] = []
			for v: int in sizes.values():
				s_arr.append(v)
			s_arr.sort()
			
			assert(s_arr.size() > 3)
			
			var last_idx: int = s_arr.size() - 1
			a1 = s_arr[last_idx] * s_arr[last_idx - 1] * s_arr[last_idx - 2]
		
		if uf.find(i) != uf.find(j):
			connections += 1
			
			if connections == (n-1):
				a2 = P[i].x * P[j].x
			
			uf.unite(i,j)
			
		
	

func part1(data: String) -> String:
	both(data)
	return str(a1)
	
func part2(data: String) -> String:
	both(data)
	return str(a2)
