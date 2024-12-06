use std::collections::BTreeSet;

pub fn process_part1(input: &str) -> i32 {
    let numbers: BTreeSet<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    for i in numbers.iter() {
        let target = 2020 - i;
        if numbers.contains(&target) {
            return i * target;
        }
    }
    0
}

pub fn sum_find(input: &str, sum: i32) -> Option<(i32, i32)> {
    let numbers: BTreeSet<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    for i in numbers.iter() {
        let target = sum - i;
        if numbers.contains(&target) {
            return Some((*i, target));
        }
    }
    None
}

pub fn process_part2(input: &str) -> i32 {
    let numbers: BTreeSet<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    for i in numbers.iter() {
        let target = 2020 - i;
        if let Some(r) = sum_find(input, target) {
            return r.0 * r.1 * i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1721
979
366
299
675
1456";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 514579);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 241861950);
    }
}
