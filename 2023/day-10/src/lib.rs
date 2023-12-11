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
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/*
#[derive(Clone, Debug, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

pub fn inbounds(point: &Point, width: usize, height: usize) -> bool {
    point.x > 0 && point.x < width && point.y > 0 && point.y < height
}

pub fn deltapoints(x: i32, y: i32, ch: char) -> Vec<(i32, i32)> {
    match ch {
        'S' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        '|' => vec![(0, -1), (0, 1)],
        '-' => vec![(-1, 0), (1, 0)],
        'L' => vec![(0, -1), (1, 0)],
        'J' => vec![(0, -1), (-1, 0)],
        '7' => vec![(0, 1), (-1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        '.' => vec![],
        _ => panic!("Unknown Char"),
    }
}

pub fn adjacent_points(p: Point, points: &Vec<Vec<char>>) -> Vec<Point> {
    let mut retval = vec![];
    for dp in deltapoints(points[p.y][p.x]) {
        let newpoint = Point {
            x: dp.x + p.x,
            y: dp.y + p.y,
        };
        if inbounds(&newpoint, points[0].len(), points.len()) {
            retval.push(newpoint);
        }
    }
    retval
}
*/

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
        assert_eq!(result, 15);
    }
}
