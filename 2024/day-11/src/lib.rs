use std::collections::HashMap;

pub fn cached_count(cache: &mut HashMap<(u64, u64), u64>, stone: u64, blink: u64) -> u64 {
    let even_length = (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0;
    let key = (stone, blink);

    // Default case
    if blink == 0 {
        return 1;
    }
    // Cached value check
    if let Some(&value) = cache.get(&key) {
        return value;
    }

    let retval = match (stone == 0, even_length) {
        (true, _) => cached_count(cache, 1, blink - 1),
        (false, false) => cached_count(cache, 2024 * stone, blink - 1),
        (false, true) => {
            let x = split(stone);
            cached_count(cache, x.0, blink - 1) + cached_count(cache, x.1, blink - 1)
        }
    };

    cache.insert(key, retval);
    retval
}

pub fn split(stone: u64) -> (u64, u64) {
    let num_digits = stone.ilog10() + 1;
    assert_eq!(num_digits % 2, 0);
    let power = 10_u64.pow(num_digits / 2);
    (stone / power, stone % power)
}

pub fn next_stones(stones: &[u128]) -> Vec<u128> {
    let mut retval = vec![];
    for &stone in stones.iter() {
        let stone_string = format!("{}", stone);
        if stone == 0 {
            retval.push(1);
        } else if stone_string.len() % 2 == 1 {
            retval.push(stone * 2024);
        } else {
            let half = stone_string.len() / 2;
            let first = &stone_string[0..half];
            let second = &stone_string[half..];
            retval.push(first.parse().unwrap());
            retval.push(second.parse().unwrap());
        }
    }
    retval
}
pub fn process_part1(input: &str) -> u128 {
    let mut stones: Vec<u128> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for _ in 0..25 {
        stones = next_stones(&stones);
    }
    stones.len() as u128
}

pub fn process_part2(input: &str) -> u64 {
    let cache = &mut HashMap::new();

    let x: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    x.iter().map(|x| cached_count(cache, *x, 75)).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 55312);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 65601038650482);
    }
}
