use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};

use pathfinding::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Bot {
    cost: usize,
    pos: (usize, usize),
    direction: usize,
}

impl Ord for Bot {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Bot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}
impl PartialEq for Bot {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Bot {}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn wallset(input: &str) -> HashSet<(usize, usize)> {
    let mut retval = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                retval.insert((row, col));
            }
        }
    }
    retval
}

pub fn end_pos(input: &str) -> (usize, usize) {
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'E' {
                return (row, col);
            }
        }
    }
    panic!("No starting point?");
}

pub fn step(pos: (usize, usize), dir: usize) -> (usize, usize) {
    let delta = DIRECTIONS[dir];
    (
        (pos.0 as i32 + delta.0) as usize,
        (pos.1 as i32 + delta.1) as usize,
    )
}

// cost, (nextpos), direction
pub fn adjacency(pos: (usize, usize), dir: usize) -> Vec<(usize, (usize, usize), usize)> {
    let mut retval = Vec::new();
    retval.push((1, step(pos, dir), dir));
    let right = (dir + 1) % 4;
    let left = (dir + 3) % 4;
    retval.push((1001, step(pos, right), right));
    retval.push((1001, step(pos, left), left));

    retval
}

pub fn start_pos(input: &str) -> (usize, usize) {
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                return (row, col);
            }
        }
    }
    panic!("No starting point?");
}

pub fn process_part1(input: &str) -> usize {
    let walls = wallset(input);
    let start = start_pos(input);
    let end = end_pos(input);

    let mut heap: BinaryHeap<Bot> = BinaryHeap::new();

    heap.push(Bot {
        cost: 0,
        pos: start,
        direction: 0,
    });

    let mut seen: HashMap<(usize, usize), usize> = HashMap::new();
    seen.insert(start, 0);

    while let Some(bot) = heap.pop() {
        if bot.pos == end {
            return bot.cost;
        }
        let adj = adjacency(bot.pos, bot.direction);
        for (cost, pos, dir) in adj.iter() {
            if !walls.contains(pos) {
                if !seen.contains_key(pos) {
                    seen.insert(*pos, *cost);
                    heap.push(Bot {
                        cost: bot.cost + cost,
                        pos: *pos,
                        direction: *dir,
                    });
                } else if seen.get(pos).unwrap() > cost {
                    seen.entry(*pos).and_modify(|key| *key = *cost);
                    heap.push(Bot {
                        cost: bot.cost + cost,
                        pos: *pos,
                        direction: *dir,
                    });
                }
            }
        }
    }

    0
}

pub fn process_part2(input: &str) -> u32 {
    let walls = wallset(input);
    let start = start_pos(input);
    let end = end_pos(input);

    // start, successors fn, heuristic fn, success fn
    let (paths, cost) = astar_bag(
        &(start, 0),
        |(position, direction)| {
            let proposed = step(*position, *direction);
            let mut adjacent: Vec<_> = Vec::new();
            if !walls.contains(&proposed) {
                adjacent.push(((proposed, *direction), 1));
            }
            adjacent.push(((*position, (*direction + 1) % 4), 1000));
            adjacent.push(((*position, (*direction + 3) % 4), 1000));
            adjacent
        },
        |_| 0,                  // Heuristic just return zero
        |&(pos, _)| pos == end, // Success condition
    )
    .expect("Someting borqd in my astar because the solution should be valid");

    dbg!(cost);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn part1a_works() {
        let result = process_part1(INPUT1);
        assert_eq!(result, 7036);
    }

    #[test]
    fn part1b_works() {
        let result = process_part1(INPUT2);
        assert_eq!(result, 11048);
    }

    #[test]
    fn part2a_works() {
        let result = process_part2(INPUT1);
        assert_eq!(result, 45);
    }

    #[test]
    fn part2b_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, 64);
    }
}
