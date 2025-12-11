class_name D11 extends RefCounted

var d: Dictionary = {}
var cache: Dictionary = {}

func populate_dictionary(data: String) -> void:
	for line: String in data.split("\n"):
		var parts: PackedStringArray = line.split(": ")
		assert(parts.size() == 2)
		var key: String = parts[0]
		var outs: PackedStringArray = parts[1].split(" ")
		d[key] = outs


func part1(data: String) -> String:
	populate_dictionary(data)
	var seen: Set = Set.new()
	var retval: int = 0

	var q: Array[String] = ["you"]
	while not q.is_empty():
		var node: String = q.pop_front()
		if seen.contains(node):
			continue
		if node == "out":
			retval += 1
		else:
			for next: String in d[node]:
				q.append(next)


	return str(retval)

func num_paths(a: Array) -> int:
	assert(a.size() == 3) # String, bool, bool
	var label: String = a[0]
	var cache_key : String = "%s_%s_%s" % [label, a[1], a[2]]
	if cache.has(cache_key):
		return cache[cache_key]

	if a[0] == "out" and a[1] and a[2]:
		cache[cache_key] = 1
		return 1
	if a[0] == "out":
		cache[cache_key] = 0
		return 0

	var retval: int = 0
	var dac: bool = a[1] or label == "dac"
	var fft: bool = a[2] or label == "fft"
	assert(d.has(label))
	for next: String in d[label]:
		retval += num_paths([next, dac, fft])
	cache[cache_key] = retval
	return retval



func part2(data: String) -> String:
	populate_dictionary(data)

	var retval: int = 0
	var start: Array = ["svr", false, false]

	retval = num_paths(start)
	print("Cache Ending Size ", cache.size())

	return str(retval)
