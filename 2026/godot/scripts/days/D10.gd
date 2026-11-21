class_name D10 extends RefCounted

func _split_lines(data: String) -> PackedStringArray:
	return data.strip_edges(true, true).split("\n", false)

func _tokenize(line: String) -> PackedStringArray:
	return line.strip_edges(true, true).split(" ", false)

func _bitmask_from_brackets(b: String) -> int:
	var result: int = 0
	var n: int = b.length()
	if n < 2:
		return 0
	for i: int in range(n-2, 0, -1):
		result *= 2
		result += int(b[i] == "#")
	return result
	
func _indices_list(token: String) -> PackedInt32Array:
	var re: RegEx = RegEx.new()
	re.compile("-?\\d+")
	var matches: Array = re.search_all(token)
	var retval: PackedInt32Array = []
	if matches == null:
		return retval
	retval.resize(matches.size())
	for i: int in range(matches.size()):
		var m: RegExMatch = matches[i]
		retval[i] = int(m.get_string())
	return retval
	
func _bitmask_from_indices(token: String) -> int:
	var retval: int = 0
	var indices: PackedInt32Array = _indices_list(token)
	for i: int in range(indices.size()):
		var v: int = indices[i]
		retval += 1 << v
	return retval

func _bfs_xor_to_zero(start_mask: int, toggles: Array[int]) -> int:
	var q: Array[Vector2i] = []
	q.append(Vector2i(start_mask, 0))
	var head: int = 0
	var visited: Set = Set.new()
	while head < q.size():
		var pair: Vector2i = q[head]
		head += 1
		var state: int = pair.x
		var steps: int = pair.y
		if visited.contains(state):
			continue
		visited.add(state)
		for wi: int in range(toggles.size()):
			var w: int = toggles[wi]
			var next_state: int = state ^ w
			if next_state == 0:
				return steps + 1
			if not visited.contains(next_state):
				q.append(Vector2i(next_state, steps + 1))
	return -1

func _state_key(vec: PackedInt32Array) -> String:
	var parts: Array[String] = []
	parts.resize(vec.size())
	for i: int in range(vec.size()):
		parts[i] = str(vec[i])
	return ",".join(parts)

func _states_equal(a: PackedInt32Array, b: PackedInt32Array) -> bool:
	if a.size() != b.size():
		return false
	for i: int in range(a.size()):
		if a[i] != b[i]:
			return false
	return true
	
func _bfs_add_to_target(ops: Array[PackedInt32Array], req: PackedInt32Array) -> int:
	var n: int = req.size()
	if n == 0:
		return 0
	
	var start: PackedInt32Array = []
	start.resize(n)
	for i:int in range(n):
		start[i] = 0
	
	var q: Array = []
	q.append([start, 0])
	var head: int = 0
	var visited: Set = Set.new()
	visited.add(_state_key(start))
	
	while head < q.size():
		var item: Array = q[head]
		head += 1
		var state: PackedInt32Array = item[0]
		var steps: int = item[1]
		
		for oi: int in range(ops.size()):
			var op: PackedInt32Array = ops[oi]
			
			var next: PackedInt32Array = []
			next.resize(n)
			for ci: int in range(n):
				next[ci] = state[ci]
			
			var valid: bool = true
			for ji: int in range(op.size()):
				var v: int = op[ji]
				if v< 0 or v >= n:
					valid = false
					break
				next[v] = next[v] + 1
				if next[v] > req[v]:
					valid = false
					break
				if not valid:
					continue
				if _states_equal(next, req):
					return steps + 1
				var key: String = _state_key(next)
				if not visited.contains(key):
					visited.add(key)
					q.append([next, steps + 1])
	return 0

func part1(data: String) -> String:
	var retval: int = 0
	var lines : PackedStringArray = _split_lines(data)
	for li: int in range(lines.size()):
		var line: String = lines[li]
		var tokens: PackedStringArray = _tokenize(line)
		
		var start_mask: int = _bitmask_from_brackets(tokens[0])
		var toggles: Array[int] = []
		for i: int in range(1, tokens.size() - 1):
			toggles.append(_bitmask_from_indices(tokens[i]))
		var steps: int = _bfs_xor_to_zero(start_mask, toggles)
		if steps >= 0:
			retval += steps
	return str(retval)
	
func part2(data: String) -> String:
	var retval: int = 0
	var lines: PackedStringArray = _split_lines(data)
	for li: int in range(lines.size()):
		var line: String = lines[li]
		var tokens: PackedStringArray = _tokenize(line)
		var ops: Array[PackedInt32Array] = []
		for i: int in range(1, tokens.size() - 1):
			ops.append(_indices_list(tokens[i]))
		var req: PackedInt32Array = _indices_list(tokens[tokens.size() - 1])
		var min_sum: int = _bfs_add_to_target(ops, req)
		if min_sum >= 0:
			retval += min_sum
	
	
	return str(retval)
