class_name AoCGraph extends RefCounted

## Perform Breadth-First Search (BFS) to find shortest path distance to a goal
## Uses layer-by-layer exploration to guarantee shortest path in unweighted graphs
## @param start: Starting node/state for the search
## @param next: Callable that takes a node and returns Array[Variant] of neighboring nodes
## @param goal: Callable that takes a node and returns bool indicating if it's the goal
## @return: Number of steps to reach goal, or -1 if goal is unreachable
static func bfs(start: Variant, next: Callable, goal: Callable) -> int:
    var q: Deque = Deque.new()
    var seen: Dictionary[Variant, bool] = {}
    q.push_back(start); seen[start] = true
    var steps: int = 0
    while not q.empty():
        var layer: int = q.size()
        while layer > 0:
            var u: Variant = q.pop_front()
            if bool(goal.call(u)): return steps
            var nbrs: Array[Variant] = next.call(u)
            for v: Variant in nbrs:
                if not seen.has(v):
                    seen[v] = true; q.push_back(v)
            layer -= 1
        steps += 1
    return -1

## Perform Dijkstra's algorithm to find shortest paths from start to all reachable nodes
## Uses priority queue to always process the node with minimum distance first
## @param start: Starting node for shortest path calculation
## @param next_costs: Callable that takes a node and returns Array[Dictionary] where each dict has:
##                    {"v": neighbor_node, "w": edge_weight}
## @return: Dictionary[Variant, int] mapping each reachable node to its shortest distance from start
##          Unreachable nodes are not included in the result
static func dijkstra(
    start: Variant,
    next_costs: Callable  # Callable(u) -> Array[Dictionary] of {"v": v, "w": weight}
) -> Dictionary[Variant, int]:
    var dist: Dictionary[Variant, int] = {}
    var pq: MinHeap = MinHeap.new()
    dist[start] = 0
    pq.push(0, start)
    while not pq.empty():
        var top: Dictionary = pq.pop()
        var d: int = int(top["p"])
        var u: Variant = top["v"]
        if d != dist.get(u, 1 << 30): continue
        var outs: Array[Dictionary] = next_costs.call(u)
        for e: Dictionary in outs:
            var v: Variant = e["v"]
            var w: int = int(e["w"])
            var nd: int = d + w
            if nd < dist.get(v, 1 << 30):
                dist[v] = nd
                pq.push(nd, v)
    return dist
