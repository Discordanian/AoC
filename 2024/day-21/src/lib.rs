use std::collections::VecDeque;
use std::collections::{BTreeMap, BTreeSet};

const NUMPAD: [char; 11] = ['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const DIRPAD: [char; 5] = ['<', 'v', '>', '^', 'A'];

pub fn map_of_moves_numpad() -> BTreeMap<(char, char), Vec<Vec<char>>> {
    let mut retval = BTreeMap::new();
    for src in NUMPAD.iter() {
        for dst in NUMPAD.iter() {
            let mut paths: Vec<Vec<char>> = Vec::new();
            retval.insert((*src, *dst), paths.clone());
            if src == dst {
                paths.push(vec![]);
                retval.insert((*src, *dst), paths);
                continue;
            }
            let mut minimum = usize::MAX;
            let mut queue: VecDeque<(char, Vec<char>)> = VecDeque::new();
            queue.push_back((*src, Vec::new()));
            while let Some((cur, v)) = queue.pop_front() {
                let mut myv = v.clone();
                myv.push(cur);
                if cur == *dst && minimum >= myv.len() {
                    minimum = myv.len();
                    // Update vec and push path onto src/dst
                    if let Some(x) = retval.get_mut(&(*src, *dst)) {
                        x.push(myv.clone());
                    }

                    continue;
                }
                if minimum < myv.len() {
                    continue;
                } else {
                    for next in adj_numpad(cur) {
                        queue.push_back((next, myv.clone()));
                    }
                }
            }
        }
    }
    retval
}

pub fn map_of_moves_dirpad() -> BTreeMap<(char, char), Vec<Vec<char>>> {
    let mut retval = BTreeMap::new();
    for src in DIRPAD.iter() {
        for dst in DIRPAD.iter() {
            let mut paths: Vec<Vec<char>> = Vec::new();
            retval.insert((*src, *dst), paths.clone());
            if src == dst {
                paths.push(vec![]);
                retval.insert((*src, *dst), paths);
                continue;
            }
            let mut minimum = usize::MAX;
            let mut queue: VecDeque<(char, Vec<char>)> = VecDeque::new();
            queue.push_back((*src, Vec::new()));
            while let Some((cur, v)) = queue.pop_front() {
                let mut myv = v.clone();
                myv.push(cur);
                if cur == *dst && minimum >= myv.len() {
                    minimum = myv.len();
                    // Update vec and push path onto src/dst
                    if let Some(x) = retval.get_mut(&(*src, *dst)) {
                        x.push(myv.clone());
                    }

                    continue;
                }
                if minimum < myv.len() {
                    continue;
                } else {
                    for next in adj_dirpad(cur) {
                        queue.push_back((next, myv.clone()));
                    }
                }
            }
        }
    }
    retval
}

pub fn input_num_vec(line: &str) -> (u64, Vec<char>) {
    let num = line[0..3].parse().unwrap();
    let v: Vec<char> = line.chars().collect();
    (num, v)
}

pub fn adj_numpad(x: char) -> Vec<char> {
    match x {
        'A' => vec!['0', '3'],
        '0' => vec!['2', 'A'],
        '1' => vec!['4', '2'],
        '2' => vec!['1', '5', '3', '0'],
        '3' => vec!['2', '6', 'A'],
        '4' => vec!['7', '5', '1'],
        '5' => vec!['4', '8', '6', '2'],
        '6' => vec!['5', '9', '3'],
        '7' => vec!['8', '4'],
        '8' => vec!['7', '9', '5'],
        '9' => vec!['8', '6'],
        _ => unreachable!("Number Adjancey got something other than 0-9A"),
    }
}

pub fn adj_dirpad(x: char) -> Vec<char> {
    match x {
        '<' => vec!['v'],
        'v' => vec!['<', '^', '>'],
        '>' => vec!['v', 'A'],
        '^' => vec!['v', 'A'],
        'A' => vec!['^', '>'],
        _ => unreachable!("Directional Adjancey got something other than <^>vA"),
    }
}

pub fn process_part1(input: &str) -> u64 {
    let in_vec: Vec<(u64, Vec<char>)> = input.lines().map(input_num_vec).collect();
    let numpad_map = map_of_moves_numpad();
    let dirpad_map = map_of_moves_dirpad();

    in_vec.len() as u64
}

pub fn process_part2(input: &str) -> u64 {
    input.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 126384);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 0);
    }
}
