use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    r: usize,
    c: usize,
    dir: usize,
    indir: i32,
}

#[derive(Clone, Eq, PartialEq)]
struct Grid {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the ordering to make the BinaryHeap act as a min heap
        other
            .r
            .cmp(&self.r)
            .then_with(|| other.c.cmp(&self.c))
            .then_with(|| other.dir.cmp(&self.dir))
            .then_with(|| other.indir.cmp(&self.indir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Grid {
    fn new(data: Vec<Vec<char>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        Grid { data, rows, cols }
    }

    fn solve(&self, part2: bool) -> i32 {
        let mut queue = BinaryHeap::new();
        let mut dist = HashMap::new();

        queue.push(State {
            r: 0,
            c: 0,
            dir: 0,
            indir: -1,
        });
        dist.insert((0, 0, 0, -1), 0);

        while let Some(State { r, c, dir, indir }) = queue.pop() {
            if let Some(&d) = dist.get(&(r, c, dir, indir)) {
                if d < dist[&(r, c, dir, indir)] {
                    continue;
                }
            }

            dist.insert((r, c, dir, indir), dist[&(r, c, dir, indir)]);

            for (i, (dr, dc)) in vec![(-1, 0), (0, 1), (1, 0), (0, -1)].iter().enumerate() {
                let rr = r as isize + dr;
                let cc = c as isize + dc;
                let new_dir = i;
                let new_indir = if new_dir != dir { indir + 1 } else { indir };

                let isnt_reverse = (new_dir + 2) % 4 != dir;
                let isvalid_part1 = new_indir <= 3;
                let isvalid_part2 =
                    new_indir <= 10 && (new_dir == dir || indir >= 4 || indir == -1);
                let isvalid = if part2 { isvalid_part2 } else { isvalid_part1 };

                if rr >= 0
                    && rr < self.rows as isize
                    && cc >= 0
                    && cc < self.cols as isize
                    && isnt_reverse
                    && isvalid
                {
                    let cost = self.data[rr as usize][cc as usize].to_digit(10).unwrap() as i32;
                    queue.push(State {
                        r: rr as usize,
                        c: cc as usize,
                        dir: new_dir,
                        indir: new_indir,
                    });
                    dist.insert(
                        (rr as usize, cc as usize, new_dir, new_indir),
                        dist[&(r, c, dir, indir)] + cost,
                    );
                }
            }
        }

        dist.into_iter()
            .filter(|&((r, c, _, _), _)| r == self.rows - 1 && c == self.cols - 1)
            .map(|(_, v)| v)
            .min()
            .unwrap_or(i32::MAX)
    }
}

fn main() {
    let content = fs::read_to_string(std::env::args().nth(1).expect("Missing file argument"))
        .expect("Failed to read file");
    let lines: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let grid = Grid::new(lines.clone());
    println!("{}", grid.solve(false));
    println!("{}", grid.solve(true));
}
