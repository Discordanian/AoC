use std::collections::{HashMap, HashSet};
use petgraph::graph::UnGraph;
use petgraph::adj::NodeIndex;

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut adjacency_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split([':', ' ', '\n']).filter(|s| !s.is_empty()).collect();

 		if let Some(node) = parts.get(0).cloned() {
            let neighbors: HashSet<&str> = parts[1..].iter().cloned().collect();
            adjacency_map.insert(node, neighbors);
        }
    }
    adjacency_map
}

fn unique_nodes(input: &str) -> HashSet<&str> {
    let mut retval = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split([':', ' ', '\n']).filter(|s| !s.is_empty()).collect();

 		if let Some(node) = parts.get(0).cloned() {
			retval.insert(node);
            for n in parts[1..].iter() {
				retval.insert(n);
			}
        }
    }
	retval
}


pub fn process_part1(input: &str) -> u32 {
	let nodelist = parse_input(input);
	let unique_list = unique_nodes(input);

	// Make an undirected graph
    let mut graph = UnGraph::<&str, u32>::default();

    let node_map: HashMap<&str, NodeIndex> = unique_list
        .iter()
        .map(|node| (*node, graph.add_node(&node)))
        .collect();

	
	// In the list I need to make sure we put edges back and forth because not all nodes are in the key
	for (key, values) in nodelist.iter() {
		for v in values.iter() {
			graph.add_edge(
			node_map[key].into(),
			node_map[v].into(),
			1
			);
		}
	}

 	// use rustworkx_core::connectivity::stoer_wagner_min_cut;

	
	input.len() as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT : &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 54);
    }
}
