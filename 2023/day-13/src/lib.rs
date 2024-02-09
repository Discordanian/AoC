pub fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() {
        return Vec::new();
    }
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    let mut transposed = Vec::with_capacity(num_cols);
    for _ in 0..num_cols {
        transposed.push(Vec::with_capacity(num_rows));
    }

    for i in 0..num_rows {
        for j in 0..num_cols {
            transposed[j].push(matrix[i][j]);
        }
    }
    transposed
}

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

fn parse(input: &str) -> Vec<VecDeque<Vec<Tile>>> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Tile::Ash,
                            '#' => Tile::Rock,
                            _ => panic!("Unknown Tile"),
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn reflects_at(grid: &VecDeque<Vec<Tile>>, smudges: usize) -> Option<usize> {
    (1..grid.len()).find(|&offset| {
        let half1 = grid.iter().take(offset).rev();
        let half2 = grid.iter().skip(offset);
        let combined = half1.zip(half2); // Check zip,
        let found_smudges: usize = combined
            .map(|(row1, row2)| row1.iter().zip(row2.iter()).filter(|(a, b)| a != b).count())
            .sum();

        found_smudges == smudges
    })
}

pub fn process_part1(input: &str) -> usize {
    let grid = parse(input);
    grid.iter()
        .map(|grid| {
            // horizontal
            if let Some(i) = reflects_at(grid, 0) {
                return i * 100;
            }

            // vertical
            let cols = (0..grid[0].len())
                .map(|i| grid.iter().map(|row| row[i]).collect())
                .collect();
            if let Some(i) = reflects_at(&cols, 0) {
                return i;
            }
            0
        })
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    let grid = parse(input);
    grid.iter()
        .map(|grid| {
            // horizontal
            if let Some(i) = reflects_at(grid, 1) {
                return i * 100;
            }

            // vertical
            let cols = (0..grid[0].len())
                .map(|i| grid.iter().map(|row| row[i]).collect())
                .collect();
            if let Some(i) = reflects_at(&cols, 1) {
                return i;
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 405);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 400);
    }
}
