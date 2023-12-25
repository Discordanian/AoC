pub fn grid_string(matrix: &Vec<Vec<char>>) -> String {
    matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .fold(String::new(), |acc, s| format!("{}\n{}", acc, s))
}

pub fn input_to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

/*
pub fn process_part1(input: &str) -> u32 {
    let mut matrix = input_to_matrix(input);
    matrix = fall_up(matrix);
    score_grid(matrix)
}
*/

pub fn score_input(input: &str) -> u32 {
    let mut matrix = input_to_matrix(input);
    dbg!(grid_string(&matrix));
    score_grid(matrix)
}

pub fn fall_up(mut matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let row_count = matrix.len();
    let col_count = matrix.iter().next().unwrap().len();

    for c in 0..col_count {
        for _ in 1..row_count {
            for r in 1..row_count {
                if matrix[r][c] == 'O' && matrix[r - 1][c] == '.' {
                    matrix[r][c] = '.';
                    matrix[r - 1][c] = 'O';
                }
            }
        }
    }
    matrix.clone()
}

pub fn score_grid(matrix: Vec<Vec<char>>) -> u32 {
    let row_count: u32 = matrix.len() as u32;
    matrix
        .iter()
        .enumerate()
        .map(|(i, &ref row)| {
            (row_count - i as u32) * (row.iter().filter(|&&x| x == 'O').count() as u32)
        })
        .sum::<u32>()
}

pub fn rotate_clockwise(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    let mut transposed = Vec::with_capacity(num_cols);
    for _ in 0..num_cols {
        transposed.push(Vec::with_capacity(num_rows));
    }

    for i in 0..num_cols {
        for j in 0..num_rows {
            let y = num_rows - j - 1;
            transposed[i].push(matrix[y][i]);
        }
    }

    transposed
}
/*
pub fn process_part2(input: &str) -> u32 {
    let mut matrix = input_to_matrix(input);
    println!("{}", grid_string(&matrix));
    for _ in 0..4 {
        matrix = fall_up(matrix);
        matrix = rotate_clockwise(matrix);
        println!("{}", grid_string(&matrix));
    }
    println!("{}", grid_string(&matrix));
    matrix.len() as u32
}
*/

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Round,
    Square,
    Empty,
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Square,
                    'O' => Tile::Round,
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect()
}

fn slide_north(grid: &mut Vec<Vec<Tile>>) {
    for col in 0..grid[0].len() {
        let mut empty_or_round_row = 0;
        for row in 0..grid.len() {
            let curr = grid[row][col];
            match curr {
                Tile::Square => empty_or_round_row = row + 1,
                Tile::Round => {
                    // swap the current tile with the empty_or_round one
                    let replace_with = std::mem::replace(&mut grid[empty_or_round_row][col], curr);
                    let _ = std::mem::replace(&mut grid[row][col], replace_with);
                    empty_or_round_row += 1;
                }
                Tile::Empty => (),
            }
        }
    }
}

fn weight(grid: &Vec<Vec<Tile>>) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| {
            let round_rocks = row.iter().filter(|tile| **tile == Tile::Round).count();
            round_rocks * (i + 1)
        })
        .sum()
}

// rotate 90 degrees clockwise: (x, y) -> (y, -x)
fn clockwise(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let size = grid.len();
    let mut rotated = vec![vec![Tile::Empty; size]; size];
    for row in 0..size {
        for col in 0..size {
            rotated[col][size - 1 - row] = grid[row][col];
        }
    }
    rotated
}

fn cycle(mut grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    for _ in 0..4 {
        slide_north(&mut grid);
        let rotated = clockwise(&grid);
        grid = rotated;
    }
    grid
}

pub fn process_part1(input: &str) -> usize {
    let mut grid = parse(input);
    slide_north(&mut grid);
    weight(&grid)
}

pub fn process_part2(input: &str) -> usize {
    let mut grid = parse(input);
    let mut seen = vec![grid.clone()];

    loop {
        grid = cycle(grid);
        // check if the cycled map has already been seen
        if let Some(idx) = seen.iter().position(|x| x == &grid) {
            // figure out length of cycle (watch out: a cycle might only start after a number of steps)
            let cycle_len = seen.len() - idx;
            // use cycle length to figure out the index of the final step in the seen list
            let final_idx = idx + (1_000_000_000 - idx) % cycle_len;
            return weight(&seen[final_idx]);
        }
        seen.push(grid.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 136);
    }

    const INPUT_SCORE: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    #[test]
    fn part1_score() {
        let result = score_input(INPUT_SCORE);
        assert_eq!(result, 136);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 64);
    }
}
