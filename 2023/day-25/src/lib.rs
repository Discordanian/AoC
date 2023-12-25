use std::collections::{HashMap, HashSet};

pub fn process_part1(input: &str) -> u32 {
	let nodelist = parse_input(input);
	dbg!(nodelist);
	input.len() as u32
}



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
