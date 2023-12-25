use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Tile {
    Empty,
    SplitHoriz,
    SplitVert,
    MirrorForward,
    MirrorBack,
}

fn input_to_grid(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '\\' => Tile::MirrorBack,
                    '/' => Tile::MirrorForward,
                    '.' => Tile::Empty,
                    '-' => Tile::SplitHoriz,
                    '|' => Tile::SplitVert,
                    _ => panic!("Unknown Tile Type"),
                })
                .collect()
        })
        .collect()
}

fn energized_count(start: Ray, grid: &[Vec<Tile>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut q = VecDeque::new();
    let mut retval = HashSet::new();
    let mut seen = HashSet::new();
    q.push_back(start);

    while let Some(mut ray) = q.pop_front() {
        if seen.contains(&ray) {
            continue;
        }
        retval.insert(ray.pos);
        seen.insert(ray);

        let dirs = match (grid[ray.pos.y][ray.pos.x], ray.dir) {
            (Tile::Empty, _)
            | (Tile::SplitHoriz, Direction::West)
            | (Tile::SplitHoriz, Direction::East)
            | (Tile::SplitVert, Direction::North)
            | (Tile::SplitVert, Direction::South) => vec![ray.dir],
            (Tile::SplitHoriz, _) => {
                vec![Direction::West, Direction::East]
            }
            (Tile::SplitVert, _) => {
                vec![Direction::North, Direction::South]
            }
            (Tile::MirrorForward, Direction::North) | (Tile::MirrorBack, Direction::South) => {
                vec![Direction::East]
            }
            (Tile::MirrorForward, Direction::South) | (Tile::MirrorBack, Direction::North) => {
                vec![Direction::West]
            }
            (Tile::MirrorForward, Direction::West) | (Tile::MirrorBack, Direction::East) => {
                vec![Direction::South]
            }
            (Tile::MirrorForward, Direction::East) | (Tile::MirrorBack, Direction::West) => {
                vec![Direction::North]
            }
        };
        for dir in dirs {
            ray.dir = dir;
            if let Some(ray) = ray.forward(rows, cols) {
                q.push_back(ray);
            }
        }
    }
    retval.len()
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ray {
    pos: Coord,
    dir: Direction,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Ray {
    fn forward(mut self, rows: usize, cols: usize) -> Option<Self> {
        match self.dir {
            Direction::North if self.pos.y > 0 => self.pos.y -= 1,
            Direction::East if self.pos.x > 0 => self.pos.x -= 1,
            Direction::South if self.pos.y < rows - 1 => self.pos.y += 1,
            Direction::West if self.pos.x < cols - 1 => self.pos.x += 1,
            _ => return None,
        }
        Some(self)
    }
}

pub fn process_part1(input: &str) -> u32 {
    let grid = input_to_grid(input);
    let start = Ray {
        pos: Coord { x: 0, y: 0 },
        dir: Direction::East,
    };
    energized_count(start, &grid) as u32
}

pub fn process_part2(input: &str) -> u32 {
    let grid = input_to_grid(input);
    let from_west = (0..grid.len()).map(|row| Ray {
        dir: Direction::East,
        pos: Coord { x: 0, y: row },
    });
    let from_east = (0..grid.len()).map(|row| Ray {
        dir: Direction::West,
        pos: Coord {
            x: grid[0].len() - 1,
            y: row,
        },
    });
    let from_north = (0..grid[0].len()).map(|col| Ray {
        dir: Direction::South,
        pos: Coord { x: col, y: 0 },
    });
    let from_south = (0..grid[0].len()).map(|col| Ray {
        dir: Direction::North,
        pos: Coord {
            x: col,
            y: grid.len() - 1,
        },
    });

    from_west
        .chain(from_east)
        .chain(from_north)
        .chain(from_south)
        .map(|start| energized_count(start, &grid))
        .max()
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        // assert_eq!(result, 46);
        assert_eq!(result, 1);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 51);
    }
}
