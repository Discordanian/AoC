// use petgraph::adj::NodeIndex;
// use petgraph::graph::UnGraph;

use petgraph::{
    algo::{self, dominators, min_spanning_tree},
    data::FromElements,
    dot::{Config, Dot},
    prelude::*,
};
use std::collections::{HashMap, HashSet};
use rustworkx_core::connectivity::stoer_wagner_min_cut;

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    let mut adjacency_map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line
            .split([':', ' ', '\n'])
            .filter(|s| !s.is_empty())
            .collect();

        if let Some(node) = parts.get(0).cloned() {
            let neighbors: HashSet<String> = parts[1..].iter().cloned().map(String::from).collect();
            adjacency_map.insert(node.to_string(), neighbors);
        }
    }
    adjacency_map
}

fn unique_nodes(input: &str) -> HashSet<String> {
    let mut retval = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line
            .split([':', ' ', '\n'])
            .filter(|s| !s.is_empty())
            .collect();

        if let Some(node) = parts.get(0).cloned() {
            retval.insert(node.to_string());
            for n in parts[1..].iter() {
                retval.insert(n.to_string());
            }
        }
    }
    retval
}

pub fn process_part1(input: &str) -> usize {
    let nodelist = parse_input(input);
    let unique_list = unique_nodes(input);

    // Make an undirected graph
    let mut graph = UnGraph::<String, u32>::default();

    let node_map: HashMap<String, NodeIndex> = unique_list
        .iter()
        .map(|node| (node.clone(), graph.add_node(node.clone())))
        .collect();

    // In the list I need to make sure we put edges back and forth because not all nodes are in the key
    for (key, values) in nodelist.iter() {
        for v in values.iter() {
            graph.add_edge(node_map[key].into(), node_map[v].into(), 1);
        }
    }

    // use rustworkx_core::connectivity::stoer_wagner_min_cut;
    let min: rustworkx_core::Result< Option<(usize, Vec<_>)>,
    > = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (_, partition_size) = min.unwrap().unwrap();

    (unique_list.len() - partition_size.len()) * partition_size.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "jqt: rhn xhk nvd
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
