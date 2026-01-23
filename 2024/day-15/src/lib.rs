use std::collections::{BTreeMap, BTreeSet};
// I like BTree over Hash because it makes debugging easier
// If you want speed, use Hash instead

pub fn make_grid1(input: &str) -> BTreeMap<(i32, i32), char> {
    let mut grid = BTreeMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid.insert((row as i32, col as i32), c);
        }
    }
    grid
    // input.lines().map(|line| line.chars().collect()).collect()
}

pub fn make_instructions(input: &str) -> Vec<char> {
    input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

pub fn bot_position(g: &BTreeMap<(i32, i32), char>) -> (i32, i32) {
    for key in g.keys() {
        if g.get(key) == Some(&'@') {
            return *key;
        }
    }
    (0, 0)
}

pub fn bot_position2(g: &[Vec<char>]) -> (usize, usize) {
    for (row, line) in g.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '@' {
                return (row, col);
            }
        }
    }
    (0, 0)
}

pub fn make_fat_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| {
            let mut row_vec: Vec<char> = Vec::new();
            for c in line.chars() {
                match c {
                    '.' => {
                        row_vec.push('.');
                        row_vec.push('.');
                    }
                    'O' => {
                        row_vec.push('[');
                        row_vec.push(']');
                    }
                    '#' => {
                        row_vec.push('#');
                        row_vec.push('#');
                    }
                    '@' => {
                        row_vec.push('@');
                        row_vec.push('.');
                    }
                    _ => panic!("Unknown character received while creating fat grid"),
                }
            }
            row_vec
        })
        .collect()
}

pub fn process_part1(input: &str) -> i32 {
    let (g, i) = input.split_once("\n\n").unwrap();

    let mut grid = make_grid1(g);
    let instructions = make_instructions(i);
    let mut botpos = bot_position(&grid);

    for dir in instructions.iter() {
        let delta = match dir {
            '^' => (-1, 0),
            'v' => (1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            _ => panic!("At the disco"),
        };
        let mut targets = vec![];

        let mut look = true;
        let mut canmove = true;
        let mut next = botpos;

        while look {
            next = (next.0 + delta.0, next.1 + delta.1);
            match grid.get(&next) {
                Some(&'O') => targets.push(next),
                Some(&'#') => {
                    canmove = false;
                    look = false;
                }
                Some(&'.') => {
                    targets.push(next);
                    look = false;
                }
                Some(x) => {
                    dbg!(x);
                    panic!("");
                }
                None => panic!("None received moving along look"),
            }
        }
        if canmove {
            // move bot to first position
            // move crate from pos one to final position
            if let Some(firstpos) = targets.first() {
                // Vacate the bot position
                if let Some(val) = grid.get_mut(&botpos) {
                    *val = '.';
                }
                botpos = *firstpos;
                // Mark first pos with bot
                if let Some(val) = grid.get_mut(firstpos) {
                    *val = '@';
                }
            }
            // If targets > 1 we have crates to move.  Just put one at the end.
            if targets.len() > 1 {
                if let Some(lastpos) = targets.last() {
                    if let Some(val) = grid.get_mut(lastpos) {
                        *val = 'O';
                    }
                }
            }
        }
    }

    score_part1(grid)
}

pub fn score_part1(grid: BTreeMap<(i32, i32), char>) -> i32 {
    grid.iter()
        .map(|(pos, v)| match v {
            'O' => pos.0 * 100 + pos.1,
            _ => 0,
        })
        .sum::<i32>()
}

pub fn score_part2(boxes: Vec<(usize, usize)>) -> usize {
    boxes.iter().map(|key| key.0 * 100 + key.1).sum()
}

pub fn print_debug(wallset: &BTreeSet<(usize, usize)>, boxset: &[(usize, usize)]) {
    let cols = wallset.iter().map(|key| key.1).max().unwrap();
    let rows = wallset.iter().map(|key| key.0).max().unwrap();

    for r in 0..rows {
        for c in 0..cols {
            let pos = (r, c);
            if wallset.contains(&pos) {
                print!("#");
            } else if boxset.contains(&pos) {
                print!("[");
            } else if c > 0 && boxset.contains(&pair_addition(pos, (0, -1))) {
                print!("]");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn pair_addition(p: (usize, usize), delta: (i32, i32)) -> (usize, usize) {
    assert!(p.0 > 0);
    assert!(p.1 > 0);
    assert!(delta.0.abs() < 2);
    let x = ((p.0 as i32) + delta.0) as usize;
    let y = ((p.1 as i32) + delta.1) as usize;
    (x, y)
}

pub fn wall_set(grid: &[Vec<char>]) -> BTreeSet<(usize, usize)> {
    let mut retval: BTreeSet<_> = BTreeSet::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '#' {
                retval.insert((row, col));
            }
        }
    }
    retval
}

pub fn vec_boxes(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut retval = Vec::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '[' {
                retval.push((row, col));
            }
        }
    }
    retval
}

pub fn process_part2(input: &str) -> usize {
    let (g, i) = input.split_once("\n\n").unwrap();

    // - For each move, build the closure of all cells that must shift (robot + both halves)
    // - If any target would push into a wall, skip the move
    // - Otherwise, shift everything simultaneously using a snapshot copy
    let mut grid = make_fat_grid(g);
    let instructions = make_instructions(i);
    let (rows, cols) = (grid.len(), grid[0].len());

    let (mut r, mut c) = bot_position2(&grid);

    for dir in instructions.iter() {
        let (dr, dc): (i32, i32) = match dir {
            '^' => (-1, 0),
            'v' => (1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            _ => panic!("At the disco"),
        };

        let mut targets: Vec<(usize, usize)> = vec![(r, c)];
        let mut go = true;

        // Iterate over `targets` while it grows (like the Python list-append loop).
        let mut idx = 0usize;
        while idx < targets.len() {
            let (cr, cc) = targets[idx];
            idx += 1;

            let nr_i = cr as i32 + dr;
            let nc_i = cc as i32 + dc;
            if nr_i < 0 || nc_i < 0 || nr_i >= rows as i32 || nc_i >= cols as i32 {
                go = false;
                break;
            }
            let (nr, nc) = (nr_i as usize, nc_i as usize);

            if targets.contains(&(nr, nc)) {
                continue;
            }

            match grid[nr][nc] {
                '#' => {
                    go = false;
                    break;
                }
                '[' => {
                    targets.push((nr, nc));
                    targets.push((nr, nc + 1));
                }
                ']' => {
                    targets.push((nr, nc));
                    targets.push((nr, nc - 1));
                }
                _ => {}
            }
        }

        if !go {
            continue;
        }

        let copy = grid.clone();

        // Move robot.
        grid[r][c] = '.';
        grid[(r as i32 + dr) as usize][(c as i32 + dc) as usize] = '@';

        // Clear all box cells involved (exclude robot at index 0).
        for &(br, bc) in targets.iter().skip(1) {
            grid[br][bc] = '.';
        }
        // Shift them from the snapshot.
        for &(br, bc) in targets.iter().skip(1) {
            let tr = (br as i32 + dr) as usize;
            let tc = (bc as i32 + dc) as usize;
            grid[tr][tc] = copy[br][bc];
        }

        r = (r as i32 + dr) as usize;
        c = (c as i32 + dc) as usize;
    }

    // Score: sum of GPS for left-half '[' positions.
    let mut sum = 0usize;
    for rr in 0..rows {
        for cc in 0..cols {
            if grid[rr][cc] == '[' {
                sum += rr * 100 + cc;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 10092);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 9021);
    }
}
