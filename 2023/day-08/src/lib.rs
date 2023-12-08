use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    left: String,
    right: String,
}

pub fn process_part1(input: &str) -> u32 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let instructions = lines[0].chars().collect::<Vec<char>>();
    // dbg!(&lines);
    // dbg!(&instructions);
    println!("{:?}", &instructions.len());
    println!("{:?}", &instructions);
    let mut node_map : HashMap::<String, Node> = HashMap::new();
    // input.split("\n").filter(|x| )
    for line in lines.iter().skip(2) {
        let name = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        println!("{} -> ({},{})", &name, &left, &right);
        node_map.insert(name.to_string(), Node {left: left.to_string(), right: right.to_string()});
    }

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

pub fn process_part2(input: &str) -> u32 {
    23
}

#[cfg(test)]
mod tests {
    use super::*;
const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 6);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 23);
    }
}
