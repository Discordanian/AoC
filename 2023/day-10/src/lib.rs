/*
from collections import deque

with open("./input.txt") as fin:
    lines = fin.read().strip().split("\n")

n, m = len(lines), len(lines[0])

def get_pairs(i, j):
    res = []
    for di, dj in list(get_delta(i, j)):
        ii, jj = i + di, j + dj
        if not (0 <= ii < n and 0 <= jj < m):
            continue
        res.append((ii, jj))
    return res

def get_delta(i, j):
    res = []
    if lines[i][j] == "S":
        for di, dj in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            ii, jj = i + di, j + dj
            if not (0 <= ii < n and 0 <= jj < m):
                continue

            if (i, j) in list(get_pairs(ii, jj)):
                res.append((di, dj))
        return res

    else:
        res = {
            "|": [(1, 0), (-1, 0)],
            "-": [(0, 1), (0, -1)],
            "L": [(-1, 0), (0, 1)],
            "J": [(-1, 0), (0, -1)],
            "7": [(1, 0), (0, -1)],
            "F": [(1, 0), (0, 1)],
            ".": [],
        }[lines[i][j]]
        return res


si, sj = None, None
for i, line in enumerate(lines):
    if "S" in line:
        si, sj = i, line.index("S")
        break


# BFS
visited = set()
dists = {}
q = deque([((si, sj), 0)])
while len(q) > 0:
    top, dist = q.popleft()
    if top in visited:
        continue
    visited.add(top)
    dists[top] = dist

    for nbr in list(get_pairs(*top)):
        if nbr in visited:
            continue
        q.append((nbr, dist + 1))

ans = max(dists.values())
print(ans)
*/
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}
impl Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row_idx: usize,
    col_idx: usize,
}

impl Coord {
    fn new(row_idx: usize, col_idx: usize) -> Self {
        Self { row_idx, col_idx }
    }

    fn valid_neighbours(&self, map: &[Vec<Tile>]) -> Vec<Coord> {
        let mut neighbours = vec![];
        let max_height = map.len() - 1;
        let max_width = map[0].len() - 1;

        match map[self.row_idx][self.col_idx] {
            Tile::Ground => (),
            Tile::Start => {
                // north
                if self.row_idx > 0 {
                    let tile = map[self.row_idx - 1][self.col_idx];
                    if matches!(tile, Tile::NorthSouth | Tile::SouthWest | Tile::SouthEast) {
                        neighbours.push(Coord::new(self.row_idx - 1, self.col_idx));
                    }
                }
                // south
                if self.row_idx < max_height {
                    let tile = map[self.row_idx + 1][self.col_idx];
                    if matches!(tile, Tile::NorthSouth | Tile::NorthWest | Tile::NorthEast) {
                        neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                    }
                }
                // west
                if self.col_idx > 0 {
                    let tile = map[self.row_idx][self.col_idx - 1];
                    if matches!(tile, Tile::EastWest | Tile::SouthEast | Tile::NorthEast) {
                        neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                    }
                }
                // east
                if self.col_idx < max_width {
                    let tile = map[self.row_idx][self.col_idx + 1];
                    if matches!(tile, Tile::EastWest | Tile::NorthWest | Tile::SouthWest) {
                        neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                    }
                }
            }
            Tile::NorthSouth => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        Tile::NorthSouth => {
                            neighbours.push(Coord::new(self.row_idx - 1, self.col_idx))
                        }
                        Tile::SouthWest => {
                            neighbours.push(Coord::new(self.row_idx - 1, self.col_idx))
                        }
                        Tile::SouthEast => {
                            neighbours.push(Coord::new(self.row_idx - 1, self.col_idx))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // south
                if self.row_idx < max_height && map[self.row_idx + 1][self.col_idx] != Tile::Ground
                {
                    match map[self.row_idx + 1][self.col_idx] {
                        Tile::NorthSouth => {
                            neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                        }
                        Tile::NorthWest => {
                            neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                        }
                        Tile::NorthEast => {
                            neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
            }
            Tile::EastWest => {
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        Tile::EastWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                        }
                        Tile::SouthEast => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                        }
                        Tile::NorthEast => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        Tile::EastWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                        }
                        Tile::NorthWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                        }
                        Tile::SouthWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
            Tile::NorthEast => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        Tile::NorthSouth => {
                            neighbours.push(Coord::new(self.row_idx - 1, self.col_idx))
                        }
                        Tile::SouthWest => {
                            neighbours.push(Coord::new(self.row_idx - 1, self.col_idx))
                        }
                        Tile::SouthEast => {
                            neighbours.push(Coord::new(self.row_idx - 1, self.col_idx))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        Tile::EastWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                        }
                        Tile::NorthWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                        }
                        Tile::SouthWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
            Tile::NorthWest => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        Tile::NorthSouth => {
                            neighbours.push(Coord::new(self.row_idx - 1, self.col_idx))
                        }
                        Tile::SouthWest => {
                            neighbours.push(Coord::new(self.row_idx - 1, self.col_idx))
                        }
                        Tile::SouthEast => {
                            neighbours.push(Coord::new(self.row_idx - 1, self.col_idx))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        Tile::EastWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                        }
                        Tile::SouthEast => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                        }
                        Tile::NorthEast => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
            }
            Tile::SouthWest => {
                // south
                if self.row_idx < max_height {
                    match map[self.row_idx + 1][self.col_idx] {
                        Tile::NorthSouth => {
                            neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                        }
                        Tile::NorthWest => {
                            neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                        }
                        Tile::NorthEast => {
                            neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        Tile::EastWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                        }
                        Tile::SouthEast => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                        }
                        Tile::NorthEast => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
            }
            Tile::SouthEast => {
                // south
                if self.row_idx < max_height {
                    match map[self.row_idx + 1][self.col_idx] {
                        Tile::NorthSouth => {
                            neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                        }
                        Tile::NorthWest => {
                            neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                        }
                        Tile::NorthEast => {
                            neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        Tile::EastWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                        }
                        Tile::NorthWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                        }
                        Tile::SouthWest => {
                            neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                        }
                        Tile::Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
        }

        neighbours
    }
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Coord) {
    let mut start = Coord::new(0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    let tile = Tile::from(c);
                    if tile == Tile::Start {
                        start = Coord::new(row_idx, col_idx)
                    }
                    tile
                })
                .collect()
        })
        .collect();
    (map, start)
}

fn build_loop(start: Coord, map: &[Vec<Tile>]) -> HashSet<Coord> {
    let mut loop_coords = HashSet::new();
    loop_coords.insert(start);
    let mut to_visit = start.valid_neighbours(map);

    while let Some(curr_pos) = to_visit.pop() {
        for neighbour in curr_pos.valid_neighbours(map) {
            if !loop_coords.contains(&neighbour) {
                to_visit.push(neighbour);
                loop_coords.insert(neighbour);
            }
        }
    }

    loop_coords
}

pub fn process_part1(input: &str) -> usize {
    let (map, start) = parse(input);
    let loop_coords = build_loop(start, &map);
    loop_coords.len() / 2
}

fn get_start_pipe(map: &[Vec<Tile>], start: Coord) -> Tile {
    let neighbours = start.valid_neighbours(map);
    let north = neighbours
        .iter()
        .find(|coord| coord.row_idx < start.row_idx)
        .is_some();
    let south = neighbours
        .iter()
        .find(|coord| coord.row_idx > start.row_idx)
        .is_some();
    let west = neighbours
        .iter()
        .find(|coord| coord.col_idx < start.col_idx)
        .is_some();
    let east = neighbours
        .iter()
        .find(|coord| coord.col_idx > start.col_idx)
        .is_some();

    match (north, west, south, east) {
        (true, true, _, _) => Tile::NorthWest,
        (true, _, true, _) => Tile::NorthSouth,
        (true, _, _, true) => Tile::NorthEast,
        (_, true, true, _) => Tile::SouthWest,
        (_, _, true, true) => Tile::SouthEast,
        (_, true, _, true) => Tile::EastWest,
        _ => panic!("Start should have exactly 2 entrances"),
    }
}

/// replace start with a valid pipe segment, and only keep pipe segments that are part of the loop
fn clean_map(start: Coord, loop_coords: &HashSet<Coord>, map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let start_pipe = get_start_pipe(&map, start);

    map.into_iter()
        .enumerate()
        .map(|(row_idx, line)| {
            line.into_iter()
                .enumerate()
                .map(|(col_idx, tile)| match tile {
                    Tile::Start => start_pipe,
                    pipe if loop_coords.contains(&Coord::new(row_idx, col_idx)) => pipe,
                    _ => Tile::Ground,
                })
                .collect()
        })
        .collect()
}

pub fn process_part2(input: &str) -> usize {
    let (map, start) = parse(input);
    let loop_coords = build_loop(start, &map);
    let map = clean_map(start, &loop_coords, map);

    let mut inside = false;

    map.into_iter()
        .flatten()
        .filter(|tile| match tile {
            Tile::Ground => inside,
            Tile::NorthSouth | Tile::NorthWest | Tile::NorthEast => {
                inside = !inside;
                false
            }
            _ => false,
        })
        .count()
}

/*
pub fn adjacent_points(p: (i32, i32), hm: &HashMap<(i32, i32), char>) -> Vec<(i32, i32)> {
    let delta_points = match hm.get(&(p.0.clone(), p.1.clone())).unwrap() {
        'S' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        '|' => vec![(0, -1), (0, 1)],
        '-' => vec![(-1, 0), (1, 0)],
        'L' => vec![(0, -1), (1, 0)],
        'J' => vec![(0, -1), (-1, 0)],
        '7' => vec![(0, 1), (-1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        '.' => vec![],
        _ => panic!("Unknown Char"),
    };
    delta_points
        .iter()
        .map(|(x, y)| (x + p.0, y + p.1))
        .collect()
}

pub fn input_to_hashmap(input: &str) -> HashMap<(i32, i32), char> {
    let mut retval = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            retval.insert((x as i32, y as i32), ch);
        }
    }
    retval
}

pub fn get_height_width(hm: &HashMap<(i32, i32), char>) -> (i32, i32) {
    let mut height = 0;
    let mut width = 0;
    for (x, y) in hm.keys() {
        width = width.max(*x);
        height = height.max(*y);
    }
    (height, width)
}

pub fn display_map(hm: &HashMap<(i32, i32), char>, width: i32, height: i32) {
    for y in 0..=height {
        for x in 0..=width {
            print!("{}", hm.get(&(x as i32, y as i32)).unwrap());
        }
        println!("");
    }
}

pub fn find_start(hm: &HashMap<(i32, i32), char>) -> (i32, i32) {
    for ((x, y), c) in hm.iter() {
        if c.clone() == 'S' {
            return (x.clone(), y.clone());
        }
    }
    (0, 0)
}

pub fn process_part1(input: &str) -> u32 {
    let pipe_map = input_to_hashmap(input);
    let (height, width) = get_height_width(&pipe_map);
    /*
    let mut seen: HashSet<Point> = HashSet::new();
    let mut dist_map: HashMap<Point, usize> = HashSet::new();
    dbg!(&points);
    */
    // dbg!(&pipe_map);
    // display_map(&pipe_map, width, height);
    // println!("S is at {:?}", find_start(&pipe_map));
    let mut dist_map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let (start_x, start_y) = find_start(&pipe_map);
    q.push_back(start_x.clone(), start_y.clone(), 0);
    dist_map.insert(find_start(&pipe_map), distance);

    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for (nx, ny) in adjacent_points((x, y), &pipe_map).iter() {}
    }
    pipe_map.len() as u32
}

pub fn process_part2(input: &str) -> u32 {
    0
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1a_works() {
        let input: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

        let result = process_part1(input);
        assert_eq!(result, 4);
    }
    #[test]
    fn part1b_works() {
        let input: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let result = process_part1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_works() {
        let input: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let result = process_part2(input);
        assert_eq!(result, 1);
    }
}
