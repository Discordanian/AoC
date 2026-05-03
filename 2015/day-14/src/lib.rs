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

fn leading_points(input: &str, race_seconds: u32) -> Vec<u32> {
    let reindeer: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(parse_reindeer)
        .collect();

    let n = reindeer.len();
    let mut points = vec![0u32; n];

    for t in 1..=race_seconds {
        let distances: Vec<u64> = reindeer
            .iter()
            .map(|&(speed, fly, rest)| distance_after(speed, fly, rest, t))
            .collect();
        let max_d = distances.iter().copied().max().unwrap_or(0);
        for (i, &d) in distances.iter().enumerate() {
            if d == max_d {
                points[i] += 1;
            }
        }
    }

    points
}

pub fn process_part2(input: &str) -> String {
    leading_points(input, RACE_SECONDS)
        .into_iter()
        .max()
        .unwrap_or(0)
        .to_string()
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

    #[test]
    fn example_part2_comet_dancer_1000s() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n\
                     Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.\n";
        let points = leading_points(input, 1000);
        assert_eq!(points[0], 312); // Comet
        assert_eq!(points[1], 689); // Dancer
    }
}
