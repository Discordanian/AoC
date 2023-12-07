pub fn process_part1(input: &str) -> String {
    let up: i32 = input.chars().filter(|x| *x == '(').count() as i32;
    let down: i32 = input.chars().filter(|x| *x == ')').count() as i32;
    (up - down).to_string()

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
