// https://www.sotr.blog/articles/breadth-first-search
use std::collections::VecDeque;

struct Queue<T> {
    pub items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, v: T) {
        self.items.push_back(v)
    }

    pub fn dequeue(&mut self) -> T {
        self.items
            .pop_front()
            .expect("Cannot dequeue from empty queue.")
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}

type Vertex = Vec<u32>;
type Graph = Vec<Vertex>;

fn bfs(graph: Graph, start_node: u32, end_node: u32) -> Option<Vec<Option<u32>>> {
    let mut queue = Queue::new();
    queue.enqueue(start_node);

    let mut visisted_vertices = vec![false; graph.len()];
    visisted_vertices[0] = true;

    let mut prev: Vec<Option<u32>> = vec![None; graph.len()];

    'outer: while !queue.is_empty() {
        let current_node = queue.dequeue();
        for v in graph[current_node as usize].iter() {
            if *v == end_node {
                prev[*v as usize] = Some(current_node);
                break 'outer;
            }

            if !visisted_vertices[*v as usize] {
                queue.enqueue(*v);
                visisted_vertices[*v as usize] = true;
                prev[*v as usize] = Some(current_node);
            }
        }
    }

    let mut path = Vec::new();
    let mut at = Some(end_node);
    while at != None {
        path.push(at);
        at = prev[at.unwrap_or(0) as usize];
    }

    path.reverse();

    return match path[0] {
        Some(x) if x == start_node => Some(path),
        _ => None,
    };
}

/* Psuedo Code

bfs arguments {
    graph: an adjacency list representation of a graph,
    start_node: integer,
    end_node: integer
}

bfs(graph, start_node, end_node) returning path from start_node to end_node if path exists, otherwise empty list {
    q = empty queue
    q.enqueue(start_node)

    visited_vertices = initialize list of false values of length |V|
    visited_vertices[0] = true

    prev = initialize list of null values of length |V|

    while queue is not empty {
        current_node = queue.dequeue()
        for each v in graph[current_node] {
            if v == end_node {
                prev[v] = current_node
                break outer loop
            }

            if !visited_vertices[v] {
                queue.enqueue(v)
                visited_vertcies[v] = true
                prev[v] = current_node
            }
        }
    }

    path = empty list
    at = end_node

    while at is not null {
        path.push(at)
        at = prev[at]
    }

    reverse(path)

    if path[0] == start_node {
        return path
    }

    return empty list

}

*/
