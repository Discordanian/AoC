pub fn process_part1(input: &str) -> u32 {
    input.len() as u32
}

pub fn score_input(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    score_grid(&matrix)
}

pub fn score_grid(matrix: &Vec<Vec<char>>) -> u32 {
    let row_count: u32 = matrix.len() as u32;
    matrix
        .iter()
        .enumerate()
        .map(|(i, &ref row)| {
            (row_count - i as u32) * (row.iter().filter(|&&x| x == 'O').count() as u32)
        })
        .sum::<u32>()
}

pub fn process_part2(input: &str) -> u32 {
    input.len() as u32
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
