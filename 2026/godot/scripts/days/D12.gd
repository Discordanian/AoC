class_name D12 extends RefCounted

func part1(data: String) -> String:
	var lines: Array = data.split("\n")
	
	var groups: Array = []
	var current_group: Array = []
	
	for line: String in lines:
		var trimmed: String = line.strip_edges()
		if trimmed.is_empty():
			if current_group.size() > 0:
				groups.append(current_group)
				current_group = []
		else:
			current_group.append(trimmed)
	
	if current_group.size() > 0:
		groups.append(current_group)
	
	if groups.is_empty():
		return "0"
	
	var R: Array = groups[-1]
	
	var s: int = 0
	var regex : RegEx = RegEx.new()
	regex.compile("-?\\d+")
	
	for row: String in R:
		var parts: PackedStringArray = row.split(" ", false)
		if parts.size() == 0:
			continue
		
		var d: String = parts[0]
		
		var matches: Array = regex.search_all(d)
		if matches.size() < 2:
			continue
		
		var w: int = int(matches[0].get_string())
		var h: int = int(matches[1].get_string())
		
		var l: Array = []
		for i: int in range(1, parts.size()):
			l.append(int(parts[i]))
		
		var sum_l: int = 0
		for val: int in l:
			sum_l += val
		
		@warning_ignore("integer_division")
		if sum_l <= (w / 3) * (h / 3):
			s += 1
	return str(s)
	
func part2(_data: String) -> String:
	return "You win"
