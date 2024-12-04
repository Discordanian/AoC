use std::collections::HashMap;

pub fn process_part1(input: &str) -> u32 {
    let mut hm = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            hm.insert((row as i32, col as i32), c);
        }
    }

    let dirs: Vec<Vec<(i32, i32)>> = vec![
        vec![(0, 1), (0, 2), (0, 3)],
        vec![(1, 1), (2, 2), (3, 3)],
        vec![(1, 0), (2, 0), (3, 0)],
        vec![(-1, 0), (-2, 0), (-3, 0)],
        vec![(0, -1), (0, -2), (0, -3)],
        vec![(1, -1), (2, -2), (3, -3)],
        vec![(-1, -1), (-2, -2), (-3, -3)],
        vec![(-1, 1), (-2, 2), (-3, 3)],
    ];

    let mut retval = 0;
    for (k, v) in hm.clone().into_iter() {
        if v == 'X' {
            for d in dirs.iter() {
                let m = hm.get(&(k.0 + d[0].0, k.1 + d[0].1));
                let a = hm.get(&(k.0 + d[1].0, k.1 + d[1].1));
                let s = hm.get(&(k.0 + d[2].0, k.1 + d[2].1));
                if m == Some(&'M') && a == Some(&'A') && s == Some(&'S') {
                    retval += 1;
                }
            }
        }
    }

    retval
}

pub fn process_part2(input: &str) -> u32 {
    let mut hm = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            hm.insert((row as i32, col as i32), c);
        }
    }

    let mut retval = 0;
    for (k, v) in hm.clone().into_iter() {
        if v == 'A' {
            let c1 = hm.get(&(k.0 + 1, k.1 + 1));
            let c2 = hm.get(&(k.0 - 1, k.1 - 1));
            let c3 = hm.get(&(k.0 + 1, k.1 - 1));
            let c4 = hm.get(&(k.0 - 1, k.1 + 1));
            let diag1: bool =
                (c1 == Some(&'M') && c2 == Some(&'S')) || (c1 == Some(&'S') && c2 == Some(&'M'));
            let diag2: bool =
                (c3 == Some(&'M') && c4 == Some(&'S')) || (c3 == Some(&'S') && c4 == Some(&'M'));
            if diag1 && diag2 {
                retval += 1;
            }
        }
    }

    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 18);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 9);
    }
}
