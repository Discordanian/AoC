use std::collections::{BTreeSet, HashMap};

const YOUR_NAME: &str = "You";

pub fn process_part1(input: &str) -> String {
    let (names, happy) = parse_happiness(input);
    max_seating(&names, &happy).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (mut names, happy) = parse_happiness(input);
    names.push(YOUR_NAME.to_string());
    max_seating(&names, &happy).to_string()
}

fn parse_happiness(input: &str) -> (Vec<String>, HashMap<(usize, usize), i32>) {
    let mut name_set: BTreeSet<String> = BTreeSet::new();
    let mut triples: Vec<(String, String, i32)> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (person, rest) = line.split_once(" would ").expect("line format");
        let (gl, right) = rest
            .split_once(" happiness units by sitting next to ")
            .expect("line format");
        let (dir, num_s) = gl.split_once(' ').expect("gain/lose");
        let num: i32 = num_s.parse().expect("number");
        let delta = match dir {
            "gain" => num,
            "lose" => -num,
            _ => panic!("expected gain or lose"),
        };
        let neighbor = right.trim_end_matches('.');
        name_set.insert(person.to_string());
        name_set.insert(neighbor.to_string());
        triples.push((person.to_string(), neighbor.to_string(), delta));
    }

    let names: Vec<String> = name_set.into_iter().collect();
    let idx: HashMap<String, usize> = names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect();

    let mut happy = HashMap::new();
    for (a, b, d) in triples {
        let ia = idx[&a];
        let ib = idx[&b];
        happy.insert((ia, ib), d);
    }

    (names, happy)
}

fn pair_score(happy: &HashMap<(usize, usize), i32>, a: usize, b: usize) -> i32 {
    happy.get(&(a, b)).copied().unwrap_or(0) + happy.get(&(b, a)).copied().unwrap_or(0)
}

fn circle_score(order: &[usize], happy: &HashMap<(usize, usize), i32>) -> i32 {
    let n = order.len();
    let mut total = 0;
    for i in 0..n {
        total += pair_score(happy, order[i], order[(i + 1) % n]);
    }
    total
}

fn next_permutation(data: &mut [usize]) -> bool {
    let i = match data.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => return false,
    };
    let j = data[i..].iter().rposition(|&x| x > data[i]).unwrap() + i;
    data.swap(i, j);
    data[i + 1..].reverse();
    true
}

fn max_seating(names: &[String], happy: &HashMap<(usize, usize), i32>) -> i32 {
    let n = names.len();
    match n {
        0 | 1 => 0,
        _ => {
            let mut best = i32::MIN;
            let mut rest: Vec<usize> = (1..n).collect();
            loop {
                let order: Vec<usize> = std::iter::once(0)
                    .chain(rest.iter().copied())
                    .collect();
                best = best.max(circle_score(&order, happy));
                if !next_permutation(&mut rest) {
                    break;
                }
            }
            best
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";

    #[test]
    fn example_is_330() {
        assert_eq!(process_part1(EXAMPLE), "330");
    }

    #[test]
    fn example_part2_with_neutral_you() {
        assert_eq!(process_part2(EXAMPLE), "286");
    }
}
