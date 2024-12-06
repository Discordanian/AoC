use std::collections::{BTreeMap, BTreeSet};

#[derive(Copy, Clone, Debug, Ord, Eq, PartialEq, PartialOrd)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Free,
    Obstacle,
}

pub fn turn_guard_right(g: (i32, i32, Direction)) -> (i32, i32, Direction) {
    match g.2 {
        Direction::East => (g.0, g.1, Direction::South),
        Direction::West => (g.0, g.1, Direction::North),
        Direction::North => (g.0, g.1, Direction::East),
        Direction::South => (g.0, g.1, Direction::West),
    }
}

pub fn loop_check(g: &(i32, i32, Direction), grid: &BTreeMap<(i32, i32), Tile>) -> bool {
    let mut guard = (g.0, g.1, g.2);
    let mut seen: BTreeSet<(i32, i32, Direction)> = BTreeSet::new();

    loop {
        // dbg!(&guard);
        let delta: (i32, i32) = match guard.2 {
            Direction::South => (0, 1),
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        };
        let next_spot = (guard.0 + delta.0, guard.1 + delta.1);
        let next_tile = grid.get(&next_spot);
        match next_tile {
            Some(Tile::Obstacle) => {
                guard = turn_guard_right(guard);
            }
            Some(Tile::Free) => {
                // Move guard
                let proposed = (next_spot.0, next_spot.1, guard.2);
                if seen.contains(&proposed) {
                    return true;
                }
                seen.insert(proposed);
                guard = (next_spot.0, next_spot.1, guard.2);
            }
            None => {
                return false;
            }
        }
    }
}

pub fn process_part1(input: &str) -> i32 {
    let mut grid: BTreeMap<(i32, i32), Tile> = BTreeMap::new();
    let mut guard = (0_i32, 0_i32, Direction::North);
    let mut seen: BTreeSet<(i32, i32)> = BTreeSet::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let row = row as i32;
            let col = col as i32;
            match c {
                '.' => {
                    grid.insert((col, row), Tile::Free);
                }
                '#' => {
                    grid.insert((col, row), Tile::Obstacle);
                }
                '^' => {
                    grid.insert((col, row), Tile::Free);
                    seen.insert((col, row));
                    guard = (col, row, Direction::North);
                }
                '>' => {
                    grid.insert((col, row), Tile::Free);
                    seen.insert((col, row));
                    guard = (col, row, Direction::East);
                }
                '<' => {
                    grid.insert((col, row), Tile::Free);
                    seen.insert((col, row));
                    guard = (col, row, Direction::West);
                }
                'v' => {
                    grid.insert((col, row), Tile::Free);
                    seen.insert((col, row));
                    guard = (col, row, Direction::South);
                }
                _ => panic!("at the disco"),
            }
        }
    }

    loop {
        // dbg!(&guard);
        let delta: (i32, i32) = match guard.2 {
            Direction::South => (0, 1),
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        };
        let next_spot = (guard.0 + delta.0, guard.1 + delta.1);
        let next_tile = grid.get(&next_spot);
        match next_tile {
            Some(Tile::Obstacle) => {
                guard = turn_guard_right(guard);
            }
            Some(Tile::Free) => {
                // Move guard
                seen.insert(next_spot);
                guard = (next_spot.0, next_spot.1, guard.2);
            }
            None => {
                return seen.len() as i32;
            }
        }
    }
}
pub fn loop_possible_set(input: &str) -> BTreeSet<(i32, i32)> {
    let mut grid: BTreeMap<(i32, i32), Tile> = BTreeMap::new();
    let mut guard = (0_i32, 0_i32, Direction::North);
    let mut seen: BTreeSet<(i32, i32)> = BTreeSet::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let row = row as i32;
            let col = col as i32;
            match c {
                '.' => {
                    grid.insert((col, row), Tile::Free);
                }
                '#' => {
                    grid.insert((col, row), Tile::Obstacle);
                }
                '^' => {
                    grid.insert((col, row), Tile::Free);
                    seen.insert((col, row));
                    guard = (col, row, Direction::North);
                }
                '>' => {
                    grid.insert((col, row), Tile::Free);
                    seen.insert((col, row));
                    guard = (col, row, Direction::East);
                }
                '<' => {
                    grid.insert((col, row), Tile::Free);
                    seen.insert((col, row));
                    guard = (col, row, Direction::West);
                }
                'v' => {
                    grid.insert((col, row), Tile::Free);
                    seen.insert((col, row));
                    guard = (col, row, Direction::South);
                }
                _ => panic!("at the disco"),
            }
        }
    }

    loop {
        // dbg!(&guard);
        let delta: (i32, i32) = match guard.2 {
            Direction::South => (0, 1),
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        };
        let next_spot = (guard.0 + delta.0, guard.1 + delta.1);
        let next_tile = grid.get(&next_spot);
        match next_tile {
            Some(Tile::Obstacle) => {
                guard = turn_guard_right(guard);
            }
            Some(Tile::Free) => {
                // Move guard
                seen.insert(next_spot);
                guard = (next_spot.0, next_spot.1, guard.2);
            }
            None => return seen,
        }
    }
}

// Optimization possible.  Instead of looking at all 'free' spaces
// Use the list of visited cells from part1
pub fn process_part2(input: &str) -> i32 {
    let mut grid: BTreeMap<(i32, i32), Tile> = BTreeMap::new();
    let mut guard = (0_i32, 0_i32, Direction::North);
    let freeset: BTreeSet<(i32, i32)> = loop_possible_set(input);

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let row = row as i32;
            let col = col as i32;
            match c {
                '.' => {
                    grid.insert((col, row), Tile::Free);
                    // freeset.insert((col, row));
                }
                '#' => {
                    grid.insert((col, row), Tile::Obstacle);
                }
                '^' => {
                    grid.insert((col, row), Tile::Free);
                    guard = (col, row, Direction::North);
                }
                '>' => {
                    grid.insert((col, row), Tile::Free);
                    guard = (col, row, Direction::East);
                }
                '<' => {
                    grid.insert((col, row), Tile::Free);
                    guard = (col, row, Direction::West);
                }
                'v' => {
                    grid.insert((col, row), Tile::Free);
                    guard = (col, row, Direction::South);
                }
                _ => panic!("at the disco"),
            }
        }
    }

    let mut ans = 0;
    for (col, row) in freeset.iter() {
        let pos = (*col, *row);
        if let Some(v) = grid.get_mut(&pos) {
            *v = Tile::Obstacle;
        }
        if loop_check(&guard, &grid) {
            ans += 1;
        }
        if let Some(v) = grid.get_mut(&pos) {
            *v = Tile::Free;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 41);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 6);
    }
}
