use std::collections::HashSet;
use std::collections::VecDeque;

pub fn pair(strpair: &str) -> (usize, usize) {
    let mut x = strpair.split(',').filter(|s| !s.is_empty());
    (
        x.next().unwrap().parse().expect("Parsable"),
        x.next().unwrap().parse().expect("Parsable"),
    )
}
pub fn make_grid(input: &str, w: usize) -> Vec<Vec<usize>> {
    let mut retval: Vec<Vec<usize>> = (0..w)
        .map(|_| (0..w).map(|_| 100_000_000).collect::<Vec<usize>>())
        .collect();
    for (i, line) in input.lines().enumerate() {
        let (c, r) = pair(line);
        if c < w && r < w {
            retval[r][c] = i + 1;
        }
    }
    retval
}

pub fn print_grid(g: &[Vec<usize>]) {
    for v in g.iter() {
        println!(
            "{}",
            v.iter().map(|x| (x % 10).to_string()).collect::<String>()
        );
    }
}

pub fn adjacent(p: (usize, usize), w: usize) -> Vec<(usize, usize)> {
    let mut retval = Vec::new();
    if p.0 > 0 {
        retval.push((p.0 - 1, p.1));
    }
    if p.1 > 0 {
        retval.push((p.0, p.1 - 1));
    }
    if p.0 < w - 1 {
        retval.push((p.0 + 1, p.1));
    }
    if p.1 < w - 1 {
        retval.push((p.0, p.1 + 1));
    }
    retval
}

pub fn process_part1(input: &str, w: usize, t: usize) -> usize {
    let grid = make_grid(input, w);
    let start = (0_usize, 0_usize);
    let end = (w - 1, w - 1);
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut deque: VecDeque<(usize, usize, usize)> = VecDeque::new();

    deque.push_back((start.0, start.1, 0));

    while let Some(x) = deque.pop_front() {
        let c = x.0;
        let r = x.1;
        let steps = x.2;
        if (r, c) == end {
            return steps;
        }
        for (nc, nr) in adjacent((c, r), w).iter() {
            let tiletime = grid[*nr][*nc];
            let tile_ok = tiletime >= (t + 1) || tiletime == 0;
            if tile_ok && !seen.contains(&(*nc, *nr)) {
                deque.push_back((*nc, *nr, steps + 1));
                seen.insert((*nc, *nr));
            }
        }
    }

    0
}
pub fn process_part2(input: &str, w: usize) -> String {
    let mut min: usize = 0;
    let mut max = input.len();
    let pairs: Vec<String> = input.lines().map(|x| x.to_string()).collect();

    // Binary search
    while min < max {
        let t: usize = (max + min) / 2;
        if process_part1(input, w, t) != 0 {
            min = t + 1;
        } else {
            max = t;
        }
    }

    pairs[min - 1].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIDTH: usize = 7;
    const TIME: usize = 12;
    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT, WIDTH, TIME);
        assert_eq!(result, 22);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT, WIDTH);
        assert_eq!(result, "6,1".to_string());
    }
}
