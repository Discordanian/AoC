class_name D02 extends RefCounted

func parse(data: String) -> Array[Array]:
	print(data)
	var retval: Array[Array] = []
	var lines: PackedStringArray = data.split(",")
	for l: String in lines:
		var ids: PackedStringArray = l.split("-")
		assert(ids.size() == 2)
		retval.append([ids[0], ids[1]])
	
	return retval

func does_repeat(n: int) -> bool:
	var n_str: String = str(n)
	if n_str.length() % 2 == 1:
		return false
	@warning_ignore("integer_division")
	var mid: int = n_str.length() / 2
	var first: String = n_str.substr(0,mid)
	var second: String = n_str.substr(mid)
	return first == second
	
func any_repeat(n: int) -> bool:
	var n_str : String = str(n)
	var l: int = n_str.length()
	@warning_ignore("integer_division")
	for i: int in range(1, (l/2) + 1):
		if l % i == 0:
			var small_str: String = n_str.substr(0,i)
			var section: String = small_str
			while small_str.length() < l:
				small_str += section

			if small_str == n_str:
				return true
	return false


func part1(data: String) -> String:
	var ids: Array[Array] = parse(data)
	var retval: int = 0
	for r: Array in ids:
		assert(r.size() == 2)
		for n: int in range(int(r[0]), int(r[1]) + 1):
			if does_repeat(n):
				retval += n
	
	
	return str(retval)
	
func part2(data: String) -> String:
	var ids: Array[Array] = parse(data)
	var retval: int = 0
	for r: Array in ids:
		assert(r.size() == 2)
		for n: int in range(int(r[0]), int(r[1]) + 1):
			if any_repeat(n):
				retval += n
	
	
	return str(retval)
