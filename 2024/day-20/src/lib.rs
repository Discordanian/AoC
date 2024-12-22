// use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

const DIRECTIONS: [IPoint; 4] = [
    IPoint { x: 1, y: 0 },
    IPoint { x: -1, y: 0 },
    IPoint { x: 0, y: 1 },
    IPoint { x: 0, y: -1 },
];

#[derive(Ord, PartialOrd, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IPoint {
    x: i32,
    y: i32,
}

const IPOINTZERO: IPoint = IPoint { x: 0, y: 0 };

impl std::ops::Add for IPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for IPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
pub fn maze_dimensions(set: &HashSet<IPoint>) -> (i32, i32) {
    let max_x = set
        .iter()
        .map(|p| p.x)
        .max()
        .expect("Set should have a max x");
    let max_y = set
        .iter()
        .map(|p| p.y)
        .max()
        .expect("Set should have a max y");
    (max_x, max_y)
}

pub fn find_start_and_end(input: &str) -> (IPoint, IPoint) {
    let mut start: IPoint = IPOINTZERO;
    let mut end: IPoint = IPOINTZERO;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = IPoint {
                    x: x as i32,
                    y: y as i32,
                }
            }
            if c == 'E' {
                end = IPoint {
                    x: x as i32,
                    y: y as i32,
                }
            }
        }
    }
    (start, end)
}

pub fn make_wall_set(input: &str) -> HashSet<IPoint> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, v)| *v == '#')
                .map(move |(x, _)| IPoint {
                    x: x as i32,
                    y: y as i32,
                }) //move required for y value in closure
        })
        .collect()
}

pub fn original_path(walls: &HashSet<IPoint>, start: IPoint, end: IPoint) -> Vec<IPoint> {
    let mut path: Vec<IPoint> = Vec::new();
    let mut seen: HashSet<IPoint> = HashSet::new();
    path.push(start); // Start at the beginning.
    seen.insert(start);

    while path.last().copied() != Some(end) {
        let pos: IPoint = *path.last().expect("Has a last");
        for delta in DIRECTIONS.iter() {
            let newpos = pos + *delta;
            if !seen.contains(&newpos) && !walls.contains(&newpos) {
                seen.insert(newpos);
                path.push(newpos);
            }
        }
    }

    path
}

pub fn process_part1(input: &str, _cheat: usize, save: usize) -> usize {
    let wallset = make_wall_set(input);
    // let _dimensions = maze_dimensions(&wallset);
    let (start, end) = find_start_and_end(input);
    let solution = original_path(&wallset, start, end);
    let mut retval = 0;

    let step_map: HashMap<IPoint, usize> =
        solution.iter().enumerate().map(|(i, p)| (*p, i)).collect();

    let two_step: Vec<IPoint> = vec![
        IPoint { x: -2, y: 0 },
        IPoint { x: -1, y: -1 },
        IPoint { x: -1, y: 1 },
        IPoint { x: 0, y: 2 },
        IPoint { x: 0, y: -2 },
        IPoint { x: 1, y: 1 },
        IPoint { x: 1, y: -1 },
        IPoint { x: 2, y: 0 },
    ];
    for p in solution.iter() {
        for delta in two_step.iter() {
            let np: IPoint = *p + *delta;
            let saved = match (step_map.get(&np), step_map.get(p)) {
                (Some(dst), Some(src)) => {
                    if dst > src {
                        dst - src
                    } else {
                        0
                    }
                }
                (_, _) => 0,
            };
            if saved > save {
                retval += 1;
            }
        }
    }
    retval
}

pub fn adjacency(p: IPoint, cheat: usize) -> Vec<(IPoint, usize)> {
    assert!(cheat > 2);
    let mut retval = Vec::new();

    for x in 0..=cheat {
        for y in 0..=cheat {
            if x + y <= cheat {
                if x != 0 {
                    retval.push((
                        p + IPoint {
                            x: x as i32,
                            y: y as i32,
                        },
                        x + y,
                    ));
                    retval.push((
                        p + IPoint {
                            x: -(x as i32),
                            y: y as i32,
                        },
                        x + y,
                    ));
                }
                if y != 0 {
                    retval.push((
                        p + IPoint {
                            x: x as i32,
                            y: y as i32,
                        },
                        x + y,
                    ));
                    retval.push((
                        p + IPoint {
                            x: x as i32,
                            y: -(y as i32),
                        },
                        x + y,
                    ));
                }
            }
        }
    }
    retval
}

pub fn process_part2(input: &str, cheat: usize, save: usize) -> u64 {
    let wallset = make_wall_set(input);
    let (start, end) = find_start_and_end(input);
    let solution = original_path(&wallset, start, end);

    let step_map: HashMap<IPoint, usize> =
        solution.iter().enumerate().map(|(i, p)| (*p, i)).collect();

    let mut p2p: HashSet<(IPoint, IPoint, i32, i32)> = HashSet::new();
    for p in solution.iter() {
        for (np, cost) in adjacency(*p, cheat).iter() {
            let saved = match (step_map.get(np), step_map.get(p)) {
                (Some(dst), Some(src)) => {
                    if dst > src {
                        dst - src
                    } else {
                        0
                    }
                }
                (_, _) => 0,
            };
            p2p.insert((*p, *np, *cost as i32, saved as i32 - *cost as i32));
        }
    }
    p2p.iter().filter(|x| x.3 >= save as i32).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHEAT1: usize = 2;
    const CHEAT2: usize = 20;
    const SAVE1: usize = 64;
    const SAVE2: usize = 20;
    const SAVE3: usize = 76;
    const SAVE4: usize = 74;
    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1a_works() {
        let result = process_part1(INPUT, CHEAT1, SAVE1);
        assert_eq!(result, 1);
    }

    #[test]
    fn part1b_works() {
        let result = process_part1(INPUT, CHEAT1, SAVE2);
        assert_eq!(result, 5);
    }

    #[test]
    fn part2a_works() {
        let result = process_part2(INPUT, CHEAT2, SAVE3);
        assert_eq!(result, 3);
    }

    #[test]
    fn part2b_works() {
        let result = process_part2(INPUT, CHEAT2, SAVE4);
        assert_eq!(result, 7);
    }

    #[test]
    fn part2c_works() {
        let result = process_part2(INPUT, CHEAT2, 72);
        assert_eq!(result, 29);
    }

    #[test]
    fn part2d_works() {
        let result = process_part2(INPUT, CHEAT2, 70);
        assert_eq!(result, 41);
    }

    #[test]
    fn part2e_works() {
        let result = process_part2(INPUT, CHEAT2, 68);
        assert_eq!(result, 55);
    }

    #[test]
    fn part2f_works() {
        let result = process_part2(INPUT, CHEAT2, 66);
        assert_eq!(result, 67);
    }
}
