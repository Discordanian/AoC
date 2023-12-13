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

pub fn process_part1(input: &str) -> u32 {
    let fields: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .map(|matrix| matrix.lines().map(|line| line.chars().collect()).collect())
        .collect();
    dbg!(&fields);
    fields.len() as u32
}

pub fn process_part2(input: &str) -> u32 {
    // 'smudge' a cell and if there is no reflection, smucde the next cell and so on.
    0
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
