use std::collections::{BTreeMap, BTreeSet};

pub fn make_grid(input: &str) -> BTreeMap<(i32, i32), i32> {
    let mut retval = BTreeMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            retval.insert((row as i32, col as i32), c.to_digit(10).unwrap() as i32);
        }
    }
    retval
}

pub fn zero_score(grid: &BTreeMap<(i32, i32), i32>, row: i32, col: i32) -> i32 {
    let mut retval = 0;
    let mut seen: BTreeSet<(i32, i32)> = BTreeSet::new();

    // down up right left
    let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    // No score for non zero starts
    if grid[&(row, col)] != 0 {
        return 0;
    }

    let mut stack: Vec<(i32, i32)> = vec![(row, col)];

    while let Some((r, c)) = stack.pop() {
        let height = grid[&(r, c)];

        // skip if already seen
        if seen.contains(&(r, c)) {
            continue;
        }
        seen.insert((r, c));

        if height == 9 {
            retval += 1;
            continue;
        }

        for (dr, dc) in directions.iter() {
            let newpair = (r + dr, c + dc);
            if grid.contains_key(&newpair) && grid[&newpair] == (height + 1) {
                stack.push(newpair);
            }
        }
    }
    retval
}

pub fn process_part1(input: &str) -> i32 {
    let grid = make_grid(input);

    grid.keys()
        .map(|(row, col)| zero_score(&grid, *row, *col))
        .sum()
}

pub fn score(grid: &BTreeMap<(i32, i32), i32>, row: i32, col: i32) -> i32 {
    let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let height = grid[&(row, col)];
    if height == 9 {
        return 1;
    }
    let mut retval = 0;

    retval += directions
        .iter()
        .map(|(dr, dc)| {
            let proposed = (dr + row, dc + col);
            match grid.contains_key(&proposed) && grid[&proposed] == height + 1 {
                true => score(grid, proposed.0, proposed.1),
                false => 0,
            }
        })
        .sum::<i32>();

    retval
}

pub fn process_part2(input: &str) -> i32 {
    let grid = make_grid(input);

    grid.iter()
        .map(|(k, v)| match *v == 0 {
            true => score(&grid, k.0, k.1),
            false => 0,
        })
        .sum()
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
        assert_eq!(result, 81);
    }
}
