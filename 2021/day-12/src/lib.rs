use std::collections::HashMap;

pub fn make_adj_map(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let mut lit = line.split('-');
        let a: String = lit.next().expect("Str slice before dash").to_string();
        let b: String = lit.next().expect("Str slice before dash").to_string();

        if b != *"start" {
            map.entry(a.clone()).or_default().push(b.clone());
        }
        if a != *"start" {
            map.entry(b.clone()).or_default().push(a);
        }
    }

    map
}

pub fn cave_is_big(a: &str) -> bool {
    a.chars().all(|c| c.is_ascii_uppercase())
}

pub fn process_part1(input: &str) -> usize {
    let map = make_adj_map(input);
    let mut retval: Vec<Vec<String>> = Vec::new();

    // (Current, Visited Vec)
    let mut stack: Vec<(String, Vec<String>)> =
        vec![(String::from("start"), vec![String::from("start")])];

    while let Some((current, visited)) = stack.pop() {
        if current == *"end" {
            retval.push(visited);
            continue;
        }
        let neighbors: Vec<String> = map[&current].clone();
        for neighbor in neighbors.iter() {
            if !visited.contains(neighbor) || cave_is_big(neighbor) {
                let mut newv: Vec<String> = visited.clone();
                newv.push(neighbor.to_string());
                stack.push((neighbor.to_string(), newv));
            }
        }
    }

    retval.len()
}

pub fn process_part2(input: &str) -> usize {
    let map = make_adj_map(input);
    let mut retval: Vec<Vec<String>> = Vec::new();

    let mut stack: Vec<(String, bool, Vec<String>)> =
        vec![(String::from("start"), false, vec![String::from("start")])];

    while let Some((current, twice, visited)) = stack.pop() {
        if current == *"end" {
            retval.push(visited);
            continue;
        }
        let neighbors: Vec<String> = map[&current].clone();
        for neighbor in neighbors.iter() {
            let neighbor_count = visited.clone().iter().filter(|x| *x == neighbor).count();
            let mut newv: Vec<String> = visited.clone();
            if cave_is_big(neighbor) {
                newv.push(neighbor.to_string());
                stack.push((neighbor.to_string(), twice, newv));
            } else {
                match (twice, neighbor_count) {
                    (_, 0) => {
                        newv.push(neighbor.to_string());
                        stack.push((neighbor.to_string(), twice, newv));
                    }
                    (false, 1) => {
                        newv.push(neighbor.to_string());
                        stack.push((neighbor.to_string(), true, newv));
                    }
                    (_, _) => {}
                }
            }
        }
    }

    retval.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUTA: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const INPUTB: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const INPUTC: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn part1a_example() {
        let result = process_part1(INPUTA);
        assert_eq!(result, 10);
    }

    #[test]
    fn part1b_example() {
        let result = process_part1(INPUTB);
        assert_eq!(result, 19);
    }

    #[test]
    fn part1c_example() {
        let result = process_part1(INPUTC);
        assert_eq!(result, 226);
    }

    #[test]
    fn part2b_example() {
        let result = process_part2(INPUTB);
        assert_eq!(result, 103);
    }

    #[test]
    fn part2c_example() {
        let result = process_part2(INPUTC);
        assert_eq!(result, 3509);
    }
}
