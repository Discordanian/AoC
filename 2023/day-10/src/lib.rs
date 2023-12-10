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

#[derive(Clone, Debug, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

pub fn inbounds(point: &Point, width: usize, height: usize) -> bool {
    point.x > 0 && point.x < width && point.y > 0 && point.y < height
}

pub fn deltapoints(ch: char) -> Vec<(i32, i32)> {
    match ch {
        'S' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        '|' => vec![(0, -1), (0, 1)],
        '-' => vec![(-1, 0), (1, 0)],
        'L' => vec![(0, -1), (1, 0)],
        'J' => vec![(0, -1), (-1, 0)],
        '7' => vec![(0, 1), (-1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        '.' => Vec![],
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

pub fn process_part1(input: &str) -> u32 {
    let points: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut seen: HashSet<Point> = HashSet::new();
    let mut dist_map: HashMap<Point, usize> = HashSet::new();
    dbg!(&points);
    points.len() as u32
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
