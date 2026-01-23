use std::collections::{HashMap, HashSet};

pub fn input_to_tuple(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let mut lit = line.split('-');
            let a = lit.next().expect("Should have a str before the dash");
            let b = lit.next().expect("Should have a str after the dash");
            (a, b)
        })
        .collect()
}
pub fn process_part1(input: &str) -> usize {
    let links = input_to_tuple(input);
    let mut graphmap: HashMap<&str, Vec<&str>> = HashMap::new();

    for (a, b) in links.iter() {
        graphmap.entry(a).or_default().push(b);
        graphmap.entry(b).or_default().push(a);
    }

    let mut set: HashSet<(&str, &str, &str)> = HashSet::new();
    for key1 in graphmap.keys() {
        for key2 in graphmap
            .get(key1)
            .expect("Key1 exists in map")
            .to_vec()
            .iter()
        {
            for key3 in graphmap
                .get(key2)
                .expect("Key2 exists in map")
                .to_vec()
                .iter()
            {
                let key3v: Vec<&str> = graphmap.get(key3).expect("Key3 exists in map").to_vec();
                if key1 != key3 && key3v.contains(key1) {
                    let mut v: Vec<&str> = vec![key1, key2, key3];
                    v.sort();
                    let tup: (&str, &str, &str) = (v[0], v[1], v[2]);
                    set.insert(tup);
                }
            }
        }
    }

    set.iter()
        .filter(|x| x.0.starts_with('t') || x.1.starts_with('t') || x.2.starts_with('t'))
        .count()
}

pub fn process_part2(input: &str) -> String {
    let links = input_to_tuple(input);
    let mut graphmap: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (a, b) in links.iter() {
        graphmap.entry(a).or_default().insert(b);
        graphmap.entry(b).or_default().insert(a);
    }

    fn search<'a>(
        node: &'a str,
        req: &mut Vec<&'a str>,
        req_set: &mut HashSet<&'a str>,
        conns: &HashMap<&'a str, HashSet<&'a str>>,
        seen: &mut HashSet<Vec<&'a str>>,
        best: &mut Vec<&'a str>,
    ) {
        let mut key = req.clone();
        key.sort_unstable();
        if !seen.insert(key.clone()) {
            return;
        }

        if key.len() > best.len() || (key.len() == best.len() && (best.is_empty() || key < *best)) {
            *best = key;
        }

        let Some(neighbors) = conns.get(node) else {
            return;
        };

        for &neighbor in neighbors.iter() {
            if req_set.contains(neighbor) {
                continue;
            }
            if !req.iter().all(|&q| {
                conns.get(q)
                    .expect("all nodes in req exist in conns")
                    .contains(neighbor)
            }) {
                continue;
            }

            req.push(neighbor);
            req_set.insert(neighbor);
            search(neighbor, req, req_set, conns, seen, best);
            req.pop();
            req_set.remove(neighbor);
        }
    }

    let mut seen: HashSet<Vec<&str>> = HashSet::new();
    let mut best: Vec<&str> = Vec::new();

    for &node in graphmap.keys() {
        let mut req = vec![node];
        let mut req_set: HashSet<&str> = HashSet::new();
        req_set.insert(node);
        search(node, &mut req, &mut req_set, &graphmap, &mut seen, &mut best);
    }

    best.sort_unstable();
    best.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 7);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "co,de,ka,ta".to_string());
    }
}
