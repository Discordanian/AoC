use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    left: String,
    right: String,
}

pub fn process_part1(input: &str) -> u32 {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let instructions = lines[0].chars().collect::<Vec<char>>();
    let node_map: HashMap<String, Node> = input
        .split('\n')
        .filter(|x| x.to_string().contains('='))
        .fold(HashMap::<String, Node>::new(), |mut hm, line| {
            hm.insert(
                line[0..3].to_string(),
                Node {
                    left: line[7..10].to_string(),
                    right: line[12..15].to_string(),
                },
            );
            hm
        });

    let mut steps = 0;
    let mut current_node = "AAA".to_string();
    let mut index = 0;
    loop {
        steps += 1;
        current_node = match instructions[index] {
            'L' => node_map[&current_node].left.clone(),
            'R' => node_map[&current_node].right.clone(),
            _ => panic!("Unknown instruction received"),
        };
        if *current_node == "ZZZ".to_string() {
            return steps;
        }
        index += 1;
        index %= instructions.len();
    }
}

pub fn step_count(node_map: HashMap<String, Node>, instructions: Vec<char>, start: String) -> u64 {
    let mut current_node = start;
    let mut index = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        current_node = match instructions[index] {
            'L' => node_map[&current_node].left.clone(),
            'R' => node_map[&current_node].right.clone(),
            _ => panic!("Unknown instruction received"),
        };
        if current_node.ends_with('Z') {
            return steps;
        }
        index += 1;
        index %= instructions.len();
    }
}

struct Ghost<'a> {
    pos: &'a str,
    cycles: Option<u64>,
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

enum Instruction {
    Left,
    Right,
}

pub fn process_part2(input: &str) -> u64 {
    let (instructions, map) = input.split_once("\n\n").unwrap();
    let instructions: Vec<Instruction> = instructions
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid Instruction"),
        })
        .collect();
    let map: HashMap<&str, Node> = map
        .lines()
        .map(|line| {
            let (source, destinations) = line.split_once(" = ").unwrap();
            let destinations = destinations
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap();
            let destinations = destinations.split_once(", ").unwrap();
            (
                source,
                Node {
                    left: String::from(destinations.0),
                    right: String::from(destinations.1),
                },
            )
        })
        .collect();

    let mut cycle_count = 0;
    let mut ghosts: Vec<Ghost> = map
        .keys()
        // start from all positions ending in 'A'
        .filter(|source| source.ends_with('A'))
        // map every location to a location with a cycle count
        .map(|pos| Ghost { pos, cycles: None })
        .collect();

    while ghosts.iter().any(|ghost| ghost.cycles.is_none()) {
        // Do a full cycle of instructions
        for ins in &instructions {
            for Ghost { pos, cycles } in ghosts.iter_mut() {
                if cycles.is_some() {
                    // this loop already has a known cycle length, no need to simulate further
                    continue;
                }
                let Node { left, right } = map.get(pos).unwrap();
                *pos = match ins {
                    Instruction::Left => left,
                    Instruction::Right => right,
                };
            }
        }
        cycle_count += 1;

        // after a full cycle of instructions, save any found cycles (ghosts that arrived at a destination)
        for Ghost { pos, cycles: cycle } in ghosts.iter_mut() {
            if cycle.is_some() {
                // already has a known cycle, no need to update
                continue;
            }
            if pos.ends_with('Z') {
                *cycle = Some(cycle_count);
            }
        }
    }

    let min_shared_cycles = ghosts
        .into_iter()
        .filter_map(|ghost| ghost.cycles)
        .fold(1, |acc, item| lcm(acc, item));

    min_shared_cycles * instructions.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT1);
        assert_eq!(result, 6);
    }

    const INPUT2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, 6);
    }
}
