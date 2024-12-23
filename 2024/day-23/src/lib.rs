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
    let mut seen: HashSet<&str> = HashSet::new();

    let mut graphmap: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut intersections: HashSet<Vec<&str>> = HashSet::new();

    for (a, b) in links.iter() {
        graphmap.entry(a).or_default().insert(b);
        graphmap.entry(a).or_default().insert(a); // map should contain itself
        graphmap.entry(b).or_default().insert(a);
        graphmap.entry(b).or_default().insert(b); // map should contain itself
    }

    for key in graphmap.keys() {
        let base: HashSet<&str> = graphmap
            .get(key)
            .expect("Key available for graphmap")
            .clone();
        // println!("A {:?}", &base);
        let mut intersection: HashSet<&str> = base.clone();
        for key2 in base.iter() {
            let mut inter2: HashSet<&str> = HashSet::new();
            let setb: HashSet<&str> = graphmap.get(key2).expect("Key2 in graphmap").clone();
            for x in setb.iter() {
                if intersection.contains(x) {
                    inter2.insert(x);
                }
            }
            /*
            println!(
                "Given set {:?} and {:?} the intersection is {:?}",
                intersection, setb, inter2
            );
            */
            intersection = inter2.clone();
            /*
            println!("B {:?}", &base);
            intersection = intersection
                .intersection(&setb)
                .map(|&x| x.clone())
                .collect();
            println!("{:?}", &intersection);
            */
            /*

            intersection = intersection .intersection(
                    &graphmap
                        .get(key2)
                        .expect("Key2 available for graphmap")
                        .clone(),
                )
                .map(|x| x.clone())
                .collect();
            */
        }
        let mut sorted = intersection.into_iter().collect::<Vec<&str>>();
        sorted.sort();

        intersections.insert(sorted);
    }

    /*
    for (a, b) in links.iter() {
        if seen.contains(a) {
            continue;
        }
        seen.insert(a);
        let mut mastervec: Vec<&str> = vec![a, b];
        let mut stack: Vec<&str> = graphmap
            .get(a)
            .expect("Create stack from first elem")
            .to_vec();
        while let Some(key) = stack.pop() {
            if !seen.contains(key) {
                seen.insert(key);
                mastervec.push(key);
                // add neighbors to stack
                for node in graphmap.get(key).expect("").to_vec().iter() {
                    stack.push(node);
                }
            }
        }
        mastervec.sort();
        mastergraphmap.insert(a, mastervec);
    }*/
    // dbg!(graphmap);

    /*
    sets = set()

    def search(node, req):
        key = tuple(sorted(req))
        if key in sets: return
        sets.add(key)
        for neighbor in conns[node]:
            if neighbor in req: continue
            if not all(neighbor in conns[query] for query in req): continue
            search(neighbor, {*req, neighbor})

    for x in conns:
        search(x, {x})

    print(",".join(sorted(max(sets, key=len))))
     */
    // dbg!(intersections);
    format!("{}", input.len())
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
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "co,de,ka,ta".to_string());
    }
}
