class_name D11 extends RefCounted

class WithPath:
    var n: String
    var p: Array[String]
    
    func _init(name: String) -> void:
        p = []
        n = name

        
    func _to_string() -> String:
        return "NodePath(%s) : %s" % [n, str(p)]

var d: Dictionary = {}

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
    
func part2(data: String) -> String:
    populate_dictionary(data)
    var seen: Set = Set.new()
    var retval: int = 0
    var start: WithPath = WithPath.new("svr")
    
    var q: Array[WithPath] = [start]
    while not q.is_empty():
        var node: WithPath = q.pop_front()
        var label: String = node.n
        var path: Array[String] = node.p
        if seen.contains(label):
            continue
        if label == "out":
            print("Path ", path)
            if path.has("fft") and path.has("dac"):
                retval += 1
        else:
            path.append(label)
            for next: String in d[label]:
                var wp: WithPath = WithPath.new(next)
                wp.p = path.duplicate()
                q.append(wp)
    
    
    return str(retval)
