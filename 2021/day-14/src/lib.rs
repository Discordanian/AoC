use std::collections::HashMap;

pub fn process_sequence(map: &HashMap<(char, char), char>, sequence: Vec<char>) -> Vec<char> {
    let mut retval: Vec<char> = Vec::new();

    assert!(!sequence.is_empty());

    for i in 1..sequence.len() {
        retval.push(sequence[i - 1]);
        retval.push(map[&(sequence[i - 1], sequence[i])]);
    }

    retval.push(sequence[sequence.len() - 1]);
    retval
}

pub fn process_part1(input: &str) -> u32 {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let mut sequence: Vec<char> = parts[0].chars().collect();
    let mut map: HashMap<(char, char), char> = HashMap::new();

    for line in parts[1].lines() {
        let liter = line.split(" -> ").collect::<Vec<_>>();
        let key1 = liter[0]
            .chars()
            .nth(0)
            .expect("A first character for the key");
        let key2 = liter[0]
            .chars()
            .nth(1)
            .expect("A second character for the key");
        let value = liter[1].chars().next().expect("Valid character as value");
        map.insert((key1, key2), value);
    }

    assert!(map.len() > 6);
    //
    for _ in 0..10 {
        sequence = process_sequence(&map, sequence);
        // dbg!(&sequence);
    }

    let minmax = minmax_sequence(&sequence);

    dbg!(minmax);

    (minmax.1 - minmax.0) as u32
}

pub fn minmax_sequence(sequence: &Vec<char>) -> (usize, usize) {
    let mut seq_max = 0;
    let mut seq_min = usize::MAX;

    for e in sequence.iter() {
        let count = sequence.into_iter().filter(|x| *x == e).count();
        if count > seq_max {
            seq_max = count;
        }
        if count < seq_min {
            seq_min = count;
        }
    }
    (seq_min, seq_max)
}

pub fn process_part2(input: &str) -> u64 {
    input.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part1_example() {
        let result = process_part1(INPUT);
        assert_eq!(result, 1588);
    }

    #[test]
    fn part2_example() {
        let result = process_part2(INPUT);
        assert_eq!(result, 2188189693529);
    }
}
