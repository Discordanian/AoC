use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

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

    while let Some(bot) = heap.pop() {
        if bot.pos == end {
            return bot.cost;
        }
        let adj = adjacency(bot.pos, bot.direction);
        for (cost, pos, dir) in adj.iter() {
            if !walls.contains(pos) {
                heap.push(Bot {
                    cost: bot.cost + cost,
                    pos: *pos,
                    direction: *dir,
                });
            }
        }
    }

    0
}

pub fn process_part2(_input: &str) -> u32 {
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
    fn part2_works() {
        let result = process_part2(INPUT1);
        assert_eq!(result, 0);
    }
}
