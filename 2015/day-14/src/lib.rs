const RACE_SECONDS: u32 = 2503;

fn distance_after(speed: u32, fly: u32, rest: u32, total_secs: u32) -> u64 {
    let cycle = fly + rest;
    let full_cycles = total_secs / cycle;
    let remainder = total_secs % cycle;
    let flying_in_remainder = remainder.min(fly);
    full_cycles as u64 * speed as u64 * fly as u64
        + speed as u64 * flying_in_remainder as u64
}

fn parse_reindeer(line: &str) -> Option<(u32, u32, u32)> {
    let nums: Vec<u32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    match nums.as_slice() {
        [speed, fly, rest] => Some((*speed, *fly, *rest)),
        _ => None,
    }
}

pub fn process_part1(input: &str) -> String {
    let best = input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(parse_reindeer)
        .map(|(speed, fly, rest)| distance_after(speed, fly, rest, RACE_SECONDS))
        .max()
        .unwrap_or(0);
    best.to_string()
}

pub fn process_part2(input: &str) -> String {
    input.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_comet_dancer_1000s() {
        let comet = distance_after(14, 10, 127, 1000);
        let dancer = distance_after(16, 11, 162, 1000);
        assert_eq!(comet, 1120);
        assert_eq!(dancer, 1056);
        assert!(comet > dancer);
    }
}
