use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn manhattan_dist(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Galaxy,
                    _ => panic!("What tile is this?"),
                })
                .collect()
        })
        .collect()
}

fn empty_rows(grid: &[Vec<Tile>]) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if !row.contains(&Tile::Galaxy) {
                Some(idx)
            } else {
                None
            }
        })
        .collect()
}

fn empty_cols(grid: &[Vec<Tile>]) -> Vec<usize> {
    let mut cols: Vec<Vec<Tile>> = vec![vec![Tile::Empty; grid.len()]; grid[0].len()];
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            cols[col_idx][row_idx] = *c;
        }
    }

    empty_rows(&cols)
}

fn galaxy_coordinates(grid: &[Vec<Tile>], expansion: usize) -> Vec<Coord> {
    let empty_rows = empty_rows(grid);
    let empty_cols = empty_cols(grid);

    let mut galaxies = Vec::new();
    let mut curr_row = 0;
    let mut curr_col = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        if empty_rows.contains(&row_idx) {
            curr_row += expansion;
            continue;
        }
        for (col_idx, c) in row.iter().enumerate() {
            if empty_cols.contains(&col_idx) {
                curr_col += expansion;
                continue;
            }
            if *c == Tile::Galaxy {
                galaxies.push(Coord {
                    row: curr_row,
                    col: curr_col,
                });
            }
            curr_col += 1
        }
        curr_col = 0;
        curr_row += 1;
    }

    galaxies
}

pub fn process_part1(input: &str) -> usize {
    let grid = parse(input);
    let galaxies = galaxy_coordinates(&grid, 2);

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| pair[0].manhattan_dist(pair[1]))
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    let grid = parse(input);
    let galaxies = galaxy_coordinates(&grid, 1_000_000);

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| pair[0].manhattan_dist(pair[1]))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 374);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 82000210);
    }
}
