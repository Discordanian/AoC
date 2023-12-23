use glam::IVec2;
use petgraph::algo::{self, all_simple_paths};
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

pub fn maxy_of_hashmap(map: &HashMap<IVec2, TileType>) -> i32 {
    let mut maxy = -1;
    for item in map.keys() {
        maxy = maxy.max(item.y);
    }

    maxy
}

pub fn process_part1(input: &str) -> u32 {
    let hm = maze_to_tiles(input);

    let mut graph = DiGraph::<&IVec2, u32>::new();

    let node_map: HashMap<IVec2, _> = hm
        .keys()
        .map(|coord| (*coord, graph.add_node(coord)))
        .collect();

    for (iv2, ttype) in hm.iter() {
        let possible_directions = match ttype {
            TileType::Directional(d) => vec![d.step(&iv2)],
            TileType::Empty => vec![
                Direction::North.step(&iv2),
                Direction::South.step(&iv2),
                Direction::East.step(&iv2),
                Direction::West.step(&iv2),
            ],
        };
        for new_iv2 in possible_directions {
            if hm.get(&new_iv2).is_some() {
                graph.add_edge(node_map[&iv2], node_map[&new_iv2], 1);
            }
        }
    }

    // dbg!(graph);
    let start_v = hm.keys().filter(|iv| iv.y == 0).next().unwrap();
    let maxy = maxy_of_hashmap(&hm);
    let finish_v = hm.keys().filter(|iv| iv.y == maxy).last().unwrap();

    let routes =
        algo::all_simple_paths::<Vec<_>, _>(&graph, node_map[start_v], node_map[finish_v], 0, None);
    // node_map.len() as u32
    routes.max_by(|a, b| a.len().cmp(&b.len())).unwrap().len() as u32 - 1_u32
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
