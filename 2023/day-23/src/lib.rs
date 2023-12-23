use glam::IVec2;
use petgraph::algo::all_simple_paths;
use petgraph::graph::DiGraph;

use std::collections::HashMap;
// use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn step(&self, position: &IVec2) -> IVec2 {
        *position
            + match self {
                Direction::North => IVec2::new(0, -1),
                Direction::South => IVec2::new(0, 1),
                Direction::East => IVec2::new(1, 0),
                Direction::West => IVec2::new(-1, 0),
            }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum TileType {
    Directional(Direction),
    Empty,
}

#[derive(Debug)]
struct Tile {
    tile_type: TileType,
}

pub fn maze_to_tiles(input: &str) -> HashMap<IVec2, TileType> {
    let mut retval = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate().filter(|(_, c)| *c != '#') {
            let ttype = match ch {
                '>' => TileType::Directional(Direction::East),
                '<' => TileType::Directional(Direction::West),
                '^' => TileType::Directional(Direction::North),
                'v' => TileType::Directional(Direction::South),
                '.' => TileType::Empty,
                _ => panic!("Tile Type Unexpected"),
            };
            retval.insert(IVec2::from((x as i32, y as i32)), ttype);
        }
    }
    retval
}

pub fn process_part1(input: &str) -> u32 {
    let hm = maze_to_tiles(input);

    let mut graph = DiGraph::<&IVec2, u32>::new();

    let node_map: HashMap<IVec2, _> = hm
        .keys()
        .map(|coord| (*coord, graph.add_node(coord)))
        .collect();

    node_map.len() as u32
}

pub fn process_part2(input: &str) -> u32 {
    input.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 94);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 154);
    }
}
