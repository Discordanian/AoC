#[derive(Clone, Debug)]
pub struct Conversion {
    dest_start: i32,
    source_start: i32,
    length: i32,
}

impl From<&str> for Conversion {
    fn from(line: &str) -> Self {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Conversion {
            dest_start: parts[0],
            source_start: parts[1],
            length: parts[2],
        }
    }
}

impl Conversion {
    fn convert(&self, seed: i32) -> i32 {
        match seed <= self.source_start || seed > (self.source_start + self.length) {
            true => seed,
            false => self.dest_start + (seed - self.source_start),
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut seeds: Vec<i32> = lines[0]
        .split(" ")
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    // dbg!(seeds);
    let conversions: Vec<Conversion> = input
        .split("\n")
        .skip(3)
        .filter(|x| !x.contains(":"))
        .filter(|x| x.len() > 0)
        .map(Conversion::from)
        .collect();
    // dbg!(conversions);
    for c in conversions {
        for seed in &mut seeds {
            *seed = c.convert(*seed);
        }
        dbg!(&c);
        dbg!(&seeds[1]);
    }
    dbg!(&seeds);
    seeds.iter().min().unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut seeds: Vec<i32> = lines[0]
        .split(" ")
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    // dbg!(seeds);
    let conversions: Vec<Conversion> = input
        .split("\n")
        .skip(3)
        .filter(|x| !x.contains(":"))
        .filter(|x| x.len() > 0)
        .map(Conversion::from)
        .collect();
    // dbg!(conversions);
    for c in conversions {
        for seed in &mut seeds {
            *seed = c.convert(*seed);
        }
        dbg!(&c);
        dbg!(&seeds[1]);
    }
    dbg!(&seeds);
    seeds.iter().min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "35".to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "46".to_string());
    }
}
