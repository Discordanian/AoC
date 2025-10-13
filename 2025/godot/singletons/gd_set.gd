class_name GDSet
extends Node

## A strongly-typed set implementation backed by a Dictionary.
## Provides O(1) add, remove, and lookup operations.

# The dictionary (map) that backs our set.
# Provides amortized O(1) adding, removing, and presence-checking.
var map: Dictionary = {}
var start: int = 0
var current: int = 0

# The dummy value we use to fill in the dictionary.
const VALUE: int = 1


func _init() -> void:
    map = {}
    start = 0
    current = 0


func should_continue() -> bool:
    return current < size()


func _iter_init(_arg: Variant) -> bool:
    current = start
    return should_continue()


func _iter_next(_arg: Variant) -> bool:
    current += 1
    return should_continue()


func _iter_get(_arg: Variant) -> Variant:
    return map.keys()[current]


func add(element: Variant) -> void:
    map[element] = VALUE


func add_all(elements: Array) -> void:
    for element: Variant in elements:
        add(element)


func remove(element: Variant) -> void:
    map.erase(element)


func remove_all(elements: Array) -> void:
    for element: Variant in elements:
        remove(element)


## Removes all elements that return true when passed to the matcher
func remove_matching(matcher: Callable) -> void:
    for element: Variant in map.keys():
        if matcher.call(element):
            remove(element)


func contains(element: Variant) -> bool:
    return map.has(element)


## Returns all elements as an array
func get_as_array() -> Array:
    return map.keys()


## Removes all elements from the set
func clear() -> void:
    map.clear()


func is_empty() -> bool:
    return map.is_empty()


func size() -> int:
    return map.size()


## Returns a new set containing all elements from this set and the other set
func union(other: GDSet) -> GDSet:
    var result: GDSet = GDSet.new()
    
    # Add all elements from this set
    for element: Variant in map.keys():
        result.add(element)
    
    # Add all elements from the other set
    for element: Variant in other.map.keys():
        result.add(element)
    
    return result


## Returns a new set containing only elements present in both sets
func intersection(other: GDSet) -> GDSet:
    var result: GDSet = GDSet.new()
    
    # Iterate over the smaller set for better performance
    var smaller_set: GDSet = self if size() <= other.size() else other
    var larger_set: GDSet = other if size() <= other.size() else self
    
    for element: Variant in smaller_set.map.keys():
        if larger_set.contains(element):
            result.add(element)
    
    return result


## Returns a new set containing elements in this set but not in the other set
func difference(other: GDSet) -> GDSet:
    var result: GDSet = GDSet.new()
    
    for element: Variant in map.keys():
        if not other.contains(element):
            result.add(element)
    
    return result


## Returns a new set containing elements in either set but not in both (symmetric difference)
func symmetric_difference(other: GDSet) -> GDSet:
    var result: GDSet = GDSet.new()
    
    # Add elements from this set not in other
    for element: Variant in map.keys():
        if not other.contains(element):
            result.add(element)
    
    # Add elements from other set not in this
    for element: Variant in other.map.keys():
        if not contains(element):
            result.add(element)
    
    return result


## Returns true if this set is a subset of the other set
func is_subset(other: GDSet) -> bool:
    if size() > other.size():
        return false
    
    for element: Variant in map.keys():
        if not other.contains(element):
            return false
    
    return true


## Returns true if this set is a superset of the other set
func is_superset(other: GDSet) -> bool:
    return other.is_subset(self)


## Returns true if the sets have no elements in common
func is_disjoint(other: GDSet) -> bool:
    var smaller_set: GDSet = self if size() <= other.size() else other
    var larger_set: GDSet = other if size() <= other.size() else self
    
    for element: Variant in smaller_set.map.keys():
        if larger_set.contains(element):
            return false
    
    return true


## Returns a string representation of the set
func _to_string() -> String:
    return "GDSet(" + str(get_as_array()) + ")"
