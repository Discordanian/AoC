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
    # Rewrite using new BigRange class
    var retval: int = 0
    var sections: PackedStringArray = data.split("\n\n")
    var rangestrs: PackedStringArray = sections[0].split("\n")
    var ranges: Array[BigRange] = []
    for r: String in rangestrs:
        var v: Array[int] = AOCUtil.array_int_from_string(r)
        assert(v.size() == 2)
        ranges.append(BigRange.new(v[0], v[1]))
    
    ranges = BigRange.merge(ranges)

    for br: BigRange in ranges:
        retval += 1 + br.end - br.begin

    return str(retval)
