class_name CustomSet
extends Resource


# The dictionary (map) that backs our set.
# Provides amortized O(1) adding, removing, and presence-checking.
var map: Dictionary
var start: int
var current: int

# The dummy value we use to fill in the dictionary.
# Having a constant makes it a little nicer to read the code and add improvements.
const VALUE = 1


func _init():
    map = Dictionary()
    self.start = 0
    self.current = 0

func should_continue():
    return (current < self.size())
    
func _iter_init(_arg):
    current = start
    return should_continue()
    
func _iter_next(_arg):
    current += 1
    return should_continue()
    
func _iter_get(_arg):
    return map.keys()[current]

func add(element):
    map[element] = VALUE


func add_all(elements):
    for element in elements:
        add(element)


func remove(element):
    map.erase(element)


func remove_all(elements):
    for element in elements:
        remove(element)


# removes all elements that return true when passed to the matcher
# can use this same pattern to implement add_matching, contains_matching, etc
func remove_matching(matcher: Callable):
    for element in map.keys():
        if matcher.call(element):
            remove(element) # safe because we're iterating over map.keys(), not map itself


func contains(element) -> bool:
    return map.has(element)


# useful when you actually *do* need to iterate over the elements
func get_as_array() -> Array:
    return map.keys()


# removes all elements from the set
func clear():
    map.clear()


func is_empty():
  return map.is_empty()


func size():
    return map.size()
