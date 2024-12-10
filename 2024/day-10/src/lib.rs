use std::collections::{BTreeMap, BTreeSet};

pub fn trail_heads(grid: &[Vec<i32>]) -> BTreeSet<(i32, i32)> {
    let mut retval = BTreeSet::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, height) in line.iter().enumerate() {
            if *height == 0 {
                retval.insert((row as i32, col as i32));
            }
        }
    }
    retval
}

pub fn process_part1(input: &str) -> u64 {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect::<Vec<Vec<i32>>>();

    let heads = trail_heads(&grid);
    // dbg!(&heads);
    grid.len() as u64
}

pub fn process_part2(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 36);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 0);
    }
}
