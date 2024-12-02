use itertools::Itertools;

pub fn process_part1(input: &str) -> u32 {
    let mut v: Vec<Vec<i32>> = vec![];

    for line in input.lines() {
        let j: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        v.push(j);
    }

    v.iter().filter(is_safe).count() as u32
}

pub fn is_safe(arr: &&Vec<i32>) -> bool {
    let mut prev = 0_i32;
    for (x, y) in arr.iter().tuple_windows() {
        let diff: i32 = x - y;
        if diff.abs() > 3 {
            return false;
        }
        if diff == 0 {
            return false;
        }
        if prev == 0 || (diff > 0 && prev > 0) {
            prev = diff;
        }
        if prev == 0 || (diff < 0 && prev < 0) {
            prev = diff;
        }
        if (prev > 0 && diff < 0) || (prev < 0 && diff > 0) {
            return false;
        }
    }
    true
}

pub fn is_safe_with_remove(arr: &&Vec<i32>) -> bool {
    for i in 0..arr.len() {
        let mut newv = arr.to_vec();
        newv.remove(i);
        if is_safe(&&newv) {
            return true;
        }
    }
    false
}

pub fn process_part2(input: &str) -> u32 {
    let mut v: Vec<Vec<i32>> = vec![];

    for line in input.lines() {
        let j: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        v.push(j);
    }

    v.iter().filter(is_safe_with_remove).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 4);
    }
}
