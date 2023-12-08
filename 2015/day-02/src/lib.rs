use itertools::Itertools;

pub fn process_part1(input: &str) -> String {
    let boxes: Vec<(u32, u32, u32)> = input
        .lines()
        .map(|line| line.split('x').map(|x| x.trim().parse().unwrap()).collect_tuple())
        .collect();
    dbg!(boxes);
    "1".to_string()
}

pub fn process_part2(input: &str) -> String {
    "1".to_string()
}
