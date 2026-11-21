class_name BigRange extends RefCounted
# BigRange is to hold Range information as an int.
# Vector2i would be nice but those ints are 32 bit int
# And Advent of Code / Everybody Codes loves 64 bit ints

var begin: int = 0
var end: int = 0

func _init(b: int, e: int) -> void:
    assert(b <= e) # Range has to be ordered correctly
    begin = b 
    end = e 

# Sort function needed for merge step
static func custom_compare(a: BigRange, b: BigRange) -> bool:
    return a.begin < b.begin

func in_range(v: int) -> bool:
    return v >= begin and v <= end

# Static function merge will take an array of BigRange and merge to reduced array set
# @param arr An Array of BigRange with overlapping ranges possible
# @return Array[BigRange] a merged Array with non-overlapping ranges
static func merge(arr: Array[BigRange]) -> Array[BigRange]:
    var retval: Array[BigRange] = []
    if arr.size() == 0:
        return []
    
    arr.sort_custom(custom_compare)
    
    var local_start: int = arr[0].begin
    var local_end: int = arr[0].end

    for i: int in range(1, arr.size()):
        var s: int = arr[i].begin
        var e: int = arr[i].end
        
        var overlap: bool = s <= local_end
        var adjacent: bool = s == (local_end + 1)
        
        if overlap or adjacent:
            if e > local_end:
                local_end = e
        else:
            retval.append(BigRange.new(local_start,local_end))
            local_start = s
            local_end = e
            
    retval.append(BigRange.new(local_start,local_end))
    return retval
    
func _to_string() -> String:
    return "{ begin: %d, end: %d }" % [begin, end]
