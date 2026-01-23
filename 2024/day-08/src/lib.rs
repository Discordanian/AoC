use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};

pub fn process_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut antinodes: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut node_map: BTreeMap<char, Vec<(i32, i32)>> = BTreeMap::new();

    let row_count = grid.len();
    let col_count = grid[0].len();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != '.' {
                let key = grid[row][col];
                let loc = vec![(row as i32, col as i32)];
                if let Entry::Vacant(e) = node_map.entry(key) {
                    e.insert(loc);
                } else if let Some(v) = node_map.get_mut(&key) {
                    v.push((row as i32, col as i32));
                }
            }
        }
    }

    for key in node_map.keys() {
        let v = node_map.get(key).unwrap();
        for (r1, c1) in v.iter() {
            for (r2, c2) in v.iter() {
                // If these are different add locations to antinodes
                let (dr, dc) = (r1 - r2, c1 - c2);
                if (dr, dc) != (0, 0) {
                    let (pr, pc) = (r1 + dr, c1 + dc);
                    if (pr, pc) != (*r1, *c1)
                        && (pr, pc) != (*r2, *c2)
                        && pr >= 0
                        && pr < (row_count as i32)
                        && pc >= 0
                        && pc < (col_count as i32)
                    {
                        antinodes.insert((pr, pc));
                    }
                }
            }
        }
    }

    antinodes.len()
}

pub fn vec_possible(rc: usize, cc: usize, p: (i32, i32), delta: (i32, i32)) -> Vec<(i32, i32)> {
    let mut retval = vec![];
    let row_limit = rc as i32;
    let col_limit = cc as i32;

    if delta.0 == 0 && delta.1 == 0 {
        return vec![];
    }

    // add delta until out of bounds
    let mut inbounds = true;
    let mut factor = 1;
    while inbounds {
        let new_row = p.0 + factor * delta.0;
        let new_col = p.1 + factor * delta.1;
        if new_row >= 0 && new_row < row_limit && new_col >= 0 && new_col < col_limit {
            retval.push((new_row, new_col));
            factor += 1;
        } else {
            inbounds = false;
        }
    }

    // sub delta until out of bounds
    let mut inbounds = true;
    let mut factor = -1;
    while inbounds {
        let new_row = p.0 + factor * delta.0;
        let new_col = p.1 + factor * delta.1;
        if new_row >= 0 && new_row < row_limit && new_col >= 0 && new_col < col_limit {
            retval.push((new_row, new_col));
            factor += -1;
        } else {
            inbounds = false;
        }
    }

    retval
}

pub fn process_part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut antinodes: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut node_map: BTreeMap<char, Vec<(i32, i32)>> = BTreeMap::new();

    let row_count = grid.len();
    let col_count = grid[0].len();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != '.' {
                let key = grid[row][col];
                let loc = vec![(row as i32, col as i32)];
                if let Entry::Vacant(e) = node_map.entry(key) {
                    e.insert(loc);
                } else if let Some(v) = node_map.get_mut(&key) {
                    v.push((row as i32, col as i32));
                }
            }
        }
    }

    for key in node_map.keys() {
        let v = node_map.get(key).unwrap();
        for (r1, c1) in v.iter() {
            for (r2, c2) in v.iter() {
                let (dr, dc) = (r1 - r2, c1 - c2);
                for (row, col) in vec_possible(row_count, col_count, (*r1, *c1), (dr, dc)).iter() {
                    antinodes.insert((*row, *col));
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 14);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 34);
    }
}
