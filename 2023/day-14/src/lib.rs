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

pub fn process_part1(input: &str) -> u32 {
    let mut matrix = input_to_matrix(input);
    matrix = fall_north(matrix);
    score_grid(matrix)
}

pub fn score_input(input: &str) -> u32 {
    let mut matrix = input_to_matrix(input);
    dbg!(grid_string(&matrix));
    score_grid(matrix)
}

pub fn fall_north(mut matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
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

pub fn rotate(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
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

pub fn process_part2(input: &str) -> u32 {
    let mut matrix = input_to_matrix(input);
    println!("{}", grid_string(&matrix));
    for _ in 0..4 {
        matrix = fall_north(matrix);
        matrix = rotate(matrix);
        println!("{}", grid_string(&matrix));
    }
    println!("{}", grid_string(&matrix));
    matrix.len() as u32
}
fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
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
