pub fn process_part1(input: &str) -> u32 {
    let fields: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .map(|matrix| matrix.lines().map(|line| line.chars().collect()).collect())
        .collect();
    dbg!(&fields);
    fields.len() as u32
}

pub fn process_part2(input: &str) -> u32 {
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
