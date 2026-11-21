class_name D03 extends RefCounted

func parse(data: String) -> Array[Array]:
	var retval: Array[Array] = []
	var strs: PackedStringArray = data.split("\n")
	for st: String in strs:
		var row: Array[int] = []
		for i: int in range(st.length()):
			row.append(int(st[i]))
		retval.append(row)
			
	return retval
 
func arr_max(arr: Array[int]) -> int:
	if arr.is_empty():
		push_error("Empty Array?")
	var maxv: int = arr[0]
	for i:int in range(1,arr.size()):
		if arr[i] > maxv:
			maxv = arr[i]
	return maxv   

func arr_to_int(arr: Array[int]) -> int:
	var retval: int = 0
	for i: int in range(arr.size()):
		retval *= 10
		retval += arr[i]
	
	return retval


func biggest_given_length(arr: Array[int], num: int) -> int:
	var retval: Array[int] = []
	var start: int = 0
	var end: int = 0

	for window: int in range(num):
		end = 1 + arr.size() - num + window
		var slice: Array[int] = arr.slice(start, end)
		var m: int = arr_max(slice)
		start = arr.find(m, start) + 1
		retval.append(m)  
	return arr_to_int(retval)

func part1(data: String) -> String:
	var retval: int = 0
	var banks: Array[Array] = parse(data)
	var start: int = 0
	for bank: Array in banks:
		start = 0
		var arr: Array[int] = []
		var slice: Array[int] = bank.slice(start,bank.size() - 1)
		var m: int = arr_max(slice)
		arr.append(m)
		start = bank.find(m)
		slice = bank.slice(start +1)
		m = arr_max(slice)
		arr.append(m) 
		retval += arr_to_int(arr) 
	return str(retval)
	
func part2(data: String) -> String:
	var banks: Array[Array] = parse(data)
	var retval: int = 0
	for bank: Array in banks:
		retval += biggest_given_length(bank, 12)
	return str(retval)
