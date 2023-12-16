pub fn process_part1(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    matrix[0].len() as u32
}

pub fn process_part2(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    matrix[1].len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 46);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 51);
    }
}
