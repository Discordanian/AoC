pub fn build_matrix(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    matrix
}

// It's a part if it's not a number or a '.'
pub fn is_part(matrix: Vec<Vec<char>>, r: usize, c: usize) -> bool {
    !matrix[r][c].is_ascii_digit() && matrix[r][c] != '.'
}

pub fn check_valid(matrix: Vec<Vec<char>>, r: usize, c: usize) -> bool {
    let min_r = match r {
        0 => 0,
        _ => r - 1,
    };
    let min_c = match c {
        0 => 0,
        _ => c - 1,
    };
    let max_r = matrix.len() - 1;
    let max_c = matrix[0].len() - 1;
    is_part(matrix.clone(), min_r, c)
        || is_part(matrix.clone(), min_r, min_c)
        || is_part(matrix.clone(), min_r, (c + 1).min(max_c))
        || is_part(matrix.clone(), (r + 1).min(max_r), c)
        || is_part(matrix.clone(), (r + 1).min(max_r), min_c)
        || is_part(matrix.clone(), (r + 1).min(max_r), (c + 1).min(max_c))
        || is_part(matrix.clone(), r, min_c)
        || is_part(matrix.clone(), r, (c + 1).min(max_c))
}

pub fn process_part1(input: &str) -> String {
    let matrix = build_matrix(input);
    let mut retval = 0;
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut value: u32 = 0;
    let mut valid = false;
    let mut in_number = false;
    for r in 0..rows {
        for c in 0..cols {
            let x = matrix[r][c];
            if !x.is_ascii_digit() && in_number && valid {
                retval += value;
                in_number = false;
                value = 0;
            }
            if !x.is_ascii_digit() && in_number && !valid {
                value = 0;
                in_number = false;
            }
            if x.is_ascii_digit() && in_number {
                value *= 10;
                value += x.to_digit(10).unwrap();
                if !valid {
                    valid = check_valid(matrix.clone(), r, c);
                }
            }
            if x.is_ascii_digit() && !in_number {
                in_number = true;
                value += x.to_digit(10).unwrap();
                valid = check_valid(matrix.clone(), r, c);
            }
            if c == cols - 1 && in_number && valid {
                retval += value;
                valid = false;
                value = 0;
            }
        }
    }

    // dbg!("{:?}",matrix);
    retval.to_string()
}

pub fn number_from_rc(matrix: Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut retval = 0;
    let mut idx = col;

    while idx > 0 && matrix[row][idx - 1].is_ascii_digit() {
        idx -= 1;
    }
    // I think idx has the start of the number now
    while idx < matrix[0].len() && matrix[row][idx].is_ascii_digit() {
        let digit = matrix[row][idx].to_digit(10).unwrap();
        retval *= 10;
        retval += digit;
        idx += 1;
    }
    retval
}

pub fn gear_pairs(matrix: Vec<Vec<char>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut retval = vec![];

    if row > 0 && col > 0 && matrix[row - 1][col - 1].is_ascii_digit() {
        retval.push((row - 1, col - 1));
    }
    if row > 0
        && matrix[row - 1][col].is_ascii_digit()
        && (col == 0 || !matrix[row - 1][col - 1].is_ascii_digit())
    {
        retval.push((row - 1, col));
    }
    if row > 0
        && col < matrix[0].len()
        && matrix[row - 1][col + 1].is_ascii_digit()
        && !matrix[row - 1][col].is_ascii_digit()
    {
        retval.push((row - 1, col + 1));
    }
    if col > 0 && matrix[row][col - 1].is_ascii_digit() {
        retval.push((row, col - 1));
    }
    if col < matrix[row].len() && matrix[row][col + 1].is_ascii_digit() {
        retval.push((row, col + 1));
    }
    if row < matrix.len() && col > 0 && matrix[row + 1][col - 1].is_ascii_digit() {
        retval.push((row + 1, col - 1));
    }
    if row < matrix.len()
        && matrix[row + 1][col].is_ascii_digit()
        && (col == 0 || !matrix[row + 1][col - 1].is_ascii_digit())
    {
        retval.push((row + 1, col));
    }
    if row < matrix.len()
        && col < matrix[0].len()
        && matrix[row + 1][col + 1].is_ascii_digit()
        && !matrix[row + 1][col].is_ascii_digit()
    {
        retval.push((row + 1, col + 1));
    }
    retval
}

pub fn process_part2(input: &str) -> String {
    let matrix = build_matrix(input);
    let mut retval = 0;
    let rows = matrix.len();
    let cols = matrix[0].len();
    for r in 0..rows {
        for c in 0..cols {
            let x = matrix[r][c];
            if x == '*' {
                let pairs = gear_pairs(matrix.clone(), r, c);
                if pairs.len() == 2 {
                    let num1 = number_from_rc(matrix.clone(), pairs[0].0, pairs[0].1);
                    let num2 = number_from_rc(matrix.clone(), pairs[1].0, pairs[1].1);
                    retval += num1 * num2;
                }
            }
        }
    }
    retval.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "4361".to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "467835".to_string());
    }
}
