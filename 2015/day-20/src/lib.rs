pub fn process_part1(input: &str) -> String {
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
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut result = input
        .split("\n\n") // Empty line between records
        .map(|record| {
            record
                .lines()
                .map(|row| row.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a,b| b.cmp(a)); // reverse sort
    let sum: u32 = result.iter().take(3).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT : &str = "1
2

5

1

3
4";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "7".to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "15".to_string());
    }
}
