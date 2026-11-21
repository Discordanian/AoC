class_name D01 extends RefCounted


func part1(data: String) -> String:
	var retval: int = 0
	var pointer: int = 50
	var instructions: PackedStringArray = data.split("\n")
	for inst: String in instructions:
		var dir: String = inst[0]
		var val: int = int(inst.substr(1))
		if dir == "L":
			pointer -= val
		else:
			pointer += val
		pointer = pointer % 100
		if pointer == 0:
			retval += 1
	return str(retval)
	
func part2(data: String) -> String:
	var retval: int = 0
	var pointer: int = 50
	var instructions: PackedStringArray = data.split("\n")
	for inst: String in instructions:
		var dir: String = inst[0]
		var val: int = int(inst.substr(1))
		@warning_ignore("integer_division")
		var full_rotations: int = val / 100
		retval += full_rotations
		val = val % 100
		print("Pointer ", pointer, " Instruction ", inst)
		if dir == "L":
			if val > pointer and pointer != 0:
				retval += 1
			pointer -= val
		else:
			if val > (100-pointer):
				retval += 1
			pointer += val
		pointer = (100 + pointer) % 100
		if pointer == 0:
			print("On zero")
			retval += 1
	return str(retval)
