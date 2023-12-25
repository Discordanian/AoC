pub fn process_part1(input: &str) -> u32 {
    let result = input
        .split("\n\n") // Empty line between records
        .map(|record| {
            record
                .lines()
                .map(|row| row.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result
}

pub fn process_part2(input: &str) -> u32 {
    let mut result = input
        .split("\n\n") // Empty line between records
        .map(|record| {
            record
                .lines()
                .map(|row| row.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a)); // reverse sort
    let sum: u32 = result.iter().take(3).sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2

5

1

3
4";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 7);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 15);
    }
}
