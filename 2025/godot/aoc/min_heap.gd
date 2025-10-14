# aoc/min_heap.gd
class_name MinHeap extends RefCounted

# Store pairs as small dictionaries {p: priority, v: value}
# Internal array-based binary heap where each element is a Dictionary with priority and value
var _a: Array[Dictionary] = []  # each: Dictionary[String, Variant]

# Check if the heap is empty
# @return: true if the heap contains no elements, false otherwise
func empty() -> bool: return _a.is_empty()

# Insert an element into the heap with given priority
# Maintains min-heap property where parent nodes have smaller or equal priority than children
# @param priority: Integer priority value (smaller values have higher priority)
# @param value: The value to store (can be any Variant type)
func push(priority: int, value: Variant) -> void:
    var node: Dictionary[String, Variant] = {"p": priority, "v": value}
    _a.append(node); _sift_up(_a.size() - 1)

# Remove and return the element with minimum priority from the heap
# @return: Dictionary with keys "p" (priority) and "v" (value) of the minimum element
# @precondition: The heap must not be empty (assertion will fail if empty)
func pop() -> Dictionary[String, Variant]:
    assert(not _a.is_empty())
    var root: Dictionary[String, Variant] = _a[0]
    var last: Dictionary[String, Variant] = _a.pop_back()
    if not _a.is_empty():
        _a[0] = last; _sift_down(0)
    return root

# Internal helper: Move element up the heap to maintain min-heap property
# Called after inserting a new element at the end of the array
# @param i: Index of the element to sift up
func _sift_up(i: int) -> void:
    var p: int = (i - 1) >> 1
    while i > 0 and int(_a[i]["p"]) < int(_a[p]["p"]):
        var t: Dictionary = _a[i]; _a[i] = _a[p]; _a[p] = t
        i = p; p = (i - 1) >> 1

# Internal helper: Move element down the heap to maintain min-heap property
# Called after removing the root element and placing the last element at the root
# @param i: Index of the element to sift down
func _sift_down(i: int) -> void:
    var n: int = _a.size()
    while true:
        var l: int = i * 2 + 1
        var r: int = l + 1
        var m: int = i
        if l < n and int(_a[l]["p"]) < int(_a[m]["p"]): m = l
        if r < n and int(_a[r]["p"]) < int(_a[m]["p"]): m = r
        if m == i: break
        var t: Dictionary = _a[i]; _a[i] = _a[m]; _a[m] = t
        i = m
