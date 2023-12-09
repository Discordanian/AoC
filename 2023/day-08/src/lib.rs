use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    left: String,
    right: String,
}

pub fn process_part1(input: &str) -> u32 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let instructions = lines[0].chars().collect::<Vec<char>>();
    println!("{:?}", &instructions.len());
    println!("{:?}", &instructions);
    let mut node_map: HashMap<String, Node> = input
        .split("\n")
        .filter(|x| x.to_string().contains("="))
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
        println!("Index {}", index);
        index %= instructions.len();
    }

    steps
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
        if current_node.ends_with("Z") {
            return steps;
        }
        index += 1;
        index %= instructions.len();
    }
    steps


}

pub fn process_part2(input: &str) -> u64 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let instructions = lines[0].chars().collect::<Vec<char>>();
    println!("{:?}", &instructions.len());
    println!("{:?}", &instructions);
    let mut node_map: HashMap<String, Node> = input
        .split("\n")
        .filter(|x| x.to_string().contains("="))
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
        println!("Index {}", index);
        index %= instructions.len();
    }
    steps
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
        assert_eq!(result, 23);
    }
}
