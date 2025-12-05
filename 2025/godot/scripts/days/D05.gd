class_name D05 extends RefCounted

func sort_f(a: Array, b: Array) -> bool:
    return a[0] < b[0]

func part1(data: String) -> String:
    var retval: int = 0
    var sections: PackedStringArray = data.split("\n\n")
    var rangestrs: PackedStringArray = sections[0].split("\n")
    var ids: Array[int] = AOCUtil.array_int_from_string(sections[1])
    var ranges: Array[Array] = []
    for r: String in rangestrs:
        var v: Array[int] = AOCUtil.array_int_from_string(r)
        assert(v.size() == 2)
        ranges.append([v[0], v[1]])

    var good: bool = false
    for id: int in ids:
        good = false
        for v: Array in ranges:
            if id >= v[0] and v[1] >= id and not good:
                print("id ", id, " range ", v)
                retval += 1
                good = true
                
        
    
    
    return str(retval)
    
func part2(data: String) -> String:
    var retval: int = 0
    var sections: PackedStringArray = data.split("\n\n")
    var rangestrs: PackedStringArray = sections[0].split("\n")
    var ranges: Array[Array] = []
    for r: String in rangestrs:
        var v: Array[int] = AOCUtil.array_int_from_string(r)
        assert(v.size() == 2)
        ranges.append([v[0], v[1]])
    

    ranges.sort_custom(sort_f)

    var merged: Array[Array] = []
    var start: int = ranges[0][0]
    var end: int = ranges[0][1]

    for i: int in range(1, ranges.size()):
        var s: int = ranges[i][0]
        var e: int = ranges[i][1]
        
        var overlap: bool = s <= end
        var adjacent: bool = s == (end + 1)
        
        if overlap or adjacent:
            if e > end:
                end = e
        else:
            merged.append([start,end])
            start = s
            end = e
            
    merged.append([start,end])

    for r: Array in merged:
        retval += r[1] - r[0] + 1
    
        
    
    
    return str(retval)
