# aoc/union_find.gd
class_name UnionFind extends RefCounted

# Used in Kruskal's Minimum Spanning Tree (MST)
# Can be used for Cycle detection.  If find(u) == find(v) before union then adding edge created a cycle

# Array storing the parent of each element (initially each element is its own parent)
var parent: Array[int]

# Array storing the rank (approximate depth) of each tree for union by rank optimization
var rank: Array[int]

# Initialize the Union-Find data structure with n elements
# Creates n disjoint sets, each containing a single element {0}, {1}, ..., {n-1}
# @param n: Number of elements to initialize (elements will be numbered 0 to n-1)
func _init(n: int) -> void:
    parent = []; rank = []
    var ok1: int = parent.resize(n); var ok2: int = rank.resize(n)
    assert(ok1 == OK and ok2 == OK)
    for i: int in n:
        parent[i] = i; rank[i] = 0

# Find the representative (root) of the set containing element x
# Uses path compression optimization to flatten the tree structure for faster future queries
# @param x: Element to find the representative of (must be in range [0, n-1])
# @return: The representative element of the set containing x
func find(x: int) -> int:
    if parent[x] != x:
        parent[x] = find(parent[x])
    return parent[x]

# Unite (merge) the sets containing elements a and b
# Uses union by rank optimization to keep trees balanced and maintain efficiency
# @param a: First element (must be in range [0, n-1])
# @param b: Second element (must be in range [0, n-1])
# @postcondition: After this operation, a and b will be in the same connected component
func unite(a: int, b: int) -> void:
    var ra: int = find(a); var rb: int = find(b)
    if ra == rb: return
    if rank[ra] < rank[rb]:
        parent[ra] = rb
    elif rank[ra] > rank[rb]:
        parent[rb] = ra
    else:
        parent[rb] = ra
        rank[ra] += 1

# Check if two elements are in the same connected component (set)
# @param a: First element (must be in range [0, n-1])
# @param b: Second element (must be in range [0, n-1])
# @return: true if a and b are in the same set, false otherwise
func same(a: int, b: int) -> bool:
    return find(a) == find(b)
