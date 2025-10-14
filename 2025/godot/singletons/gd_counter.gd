class_name Counter
extends RefCounted

var _counts: Dictionary = {}

func _init(items: Array = []):
    for item in items:
        add(item)

# Add one occurrence of an item
func add(item: Variant, count: int = 1) -> void:
    if _counts.has(item):
        _counts[item] += count
    else:
        _counts[item] = count

# Get count of an item (returns 0 if not present)
func get_count(item: Variant) -> int:
    return _counts.get(item, 0)

# Remove occurrences of an item
func remove(item: Variant, count: int = 1) -> void:
    if _counts.has(item):
        _counts[item] -= count
        if _counts[item] <= 0:
            _counts.erase(item)

# Get the most common items
func most_common(n: int = -1) -> Array:
    var items: Array = []
    for key in _counts.keys():
        items.append([key, _counts[key]])
    
    # Sort by count (descending)
    items.sort_custom(func(a, b): return a[1] > b[1])
    
    if n > 0:
        return items.slice(0, n)
    return items

# Get total count of all items
func total() -> int:
    var sum: int = 0
    for count in _counts.values():
        sum += count
    return sum

# Get all unique items
func items() -> Array:
    return _counts.keys()

# Clear all counts
func clear() -> void:
    _counts.clear()

# Check if item exists
func has(item: Variant) -> bool:
    return _counts.has(item)

# Get the underlying dictionary
func to_dict() -> Dictionary:
    return _counts.duplicate()

func _to_string() -> String:
    return str(_counts)
