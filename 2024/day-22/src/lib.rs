use std::collections::{HashMap, HashSet};
const MONKEYMOD: u64 = 16777216;

pub fn next(n: u64) -> u64 {
    let mut retval = (n ^ (n * 64)) % MONKEYMOD;
    retval = (retval ^ (retval / 32)) % MONKEYMOD;
    retval = (retval ^ (retval * 2048)) % MONKEYMOD;

    retval
}
pub fn process_part1(input: &str) -> u64 {
    let prices: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut retval = 0;

    for p in prices.iter() {
        let mut p2000 = *p;
        for _ in 0..2000 {
            p2000 = next(p2000);
        }
        retval += p2000;
    }

    retval
}

use itertools::Itertools;
pub fn process_part2(input: &str) -> i64 {
    let prices: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();

    let mut map: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

    for p in prices.iter() {
        let mut buyers: Vec<i64> = Vec::new();
        let mut p2000 = *p;
        buyers.push(p2000 % 10);
        for _ in 0..2000 {
            p2000 = next(p2000 as u64) as i64;
            buyers.push(p2000 % 10);
        }
        let mut seen: HashSet<(i64, i64, i64, i64)> = HashSet::new();
        for (a, b, c, d, e) in buyers.iter().tuple_windows() {
            let changes: (i64, i64, i64, i64) = (b - a, c - b, d - c, e - d);
            if !seen.contains(&changes) {
                seen.insert(changes);
                *map.entry(changes).or_default() += e;
            }
        }
    }

    // dbg!(&map);

    *map.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUTSMALL: &str = "1";

    const INPUT: &str = "1
10
100
2024";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn part2small_works() {
        let result = process_part2(INPUTSMALL);
        assert_eq!(result, 9);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 24);
    }
}
