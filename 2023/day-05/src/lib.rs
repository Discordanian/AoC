use itertools::Itertools;

#[derive(Clone, Debug)]
struct Conversion {
    dest_start: i64,
    source_start: i64,
    length: i64,
}

#[derive(Clone, Debug)]
struct Range {
    from: i64,
    to: i64,
}

impl From<&str> for Conversion {
    fn from(line: &str) -> Self {
        let parts: Vec<i64> = line
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
    fn convert(&self, seed: i64) -> i64 {
        match seed <= self.source_start || seed > (self.source_start + self.length) {
            true => seed,
            false => self.dest_start + (seed - self.source_start),
        }
    }
}

pub fn process_part1(input: &str) -> i64 {
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
    let seeds = seeds_str.strip_prefix("seeds: ").unwrap();
    let seeds = seeds.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let mut maps = Vec::new();
    for block in maps_str.split("\n\n") {
        let (_, rules) = block.split_once("\n").unwrap();
        let mut map = Vec::new();
        for line in rules.lines() {
            let mut nums = line.splitn(3, " ");
            let dest_start: i64 = nums.next().unwrap().parse().unwrap();
            let source_start: i64 = nums.next().unwrap().parse().unwrap();
            let length: i64 = nums.next().unwrap().parse().unwrap();
            map.push(Conversion {
                dest_start,
                source_start,
                length,
            });
        }
        maps.push(map);
    }

    let mut min = i64::MAX;

    for seed in seeds {
        let mut curr = seed;

        'maps: for map in &maps {
            for conversion in map {
                let rule_applies =
                    curr >= conversion.source_start && curr <= conversion.source_start + conversion.length;
                if rule_applies {
                    let offset = curr - conversion.source_start;
                    curr = conversion.dest_start + offset;
                    continue 'maps;
                }
            }
        }

        min = min.min(curr);
    }
    min
}

pub fn process_part2(input: &str) -> i64 {
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
    let seeds = seeds_str.strip_prefix("seeds: ").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .chunks(2); // Itertools
    let seeds = seeds.into_iter().map(|mut chunk| {
        let from = chunk.next().unwrap();
        let range = chunk.next().unwrap();
        Range {
            from,
            to: from + range,
        }
    });

    let maps: Vec<Vec<Conversion>> = maps_str
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.splitn(3, " ");
                    Conversion {
                        dest_start: nums.next().unwrap().parse().unwrap(),
                        source_start: nums.next().unwrap().parse().unwrap(),
                        length: nums.next().unwrap().parse().unwrap(),
                    }
                })
                .sorted_by(|a, b| a.source_start.cmp(&b.source_start))
                .collect()
        })
        .collect();

    let mut curr_ranges: Vec<Range> = seeds.collect();

    for map in &maps {
        let mut new_ranges: Vec<Range> = Vec::new();

        for range in &curr_ranges {
            let mut curr = range.clone();

            for rule in map {
                let offset = rule.dest_start - rule.source_start;
                let rule_applies = curr.from <= curr.to
                    && curr.from <= rule.source_start + rule.length
                    && curr.to >= rule.source_start;

                if rule_applies {
                    if curr.from < rule.source_start {
                        new_ranges.push(Range {
                            from: curr.from,
                            to: rule.source_start - 1,
                        });
                        curr.from = rule.source_start;
                        if curr.to < rule.source_start + rule.length {
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: curr.to + offset,
                            });
                            curr.from = curr.to + 1;
                        } else {
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: rule.source_start + rule.length - 1 + offset,
                            });
                            curr.from = rule.source_start + rule.length;
                        }
                    } else if curr.to < rule.source_start + rule.length {
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: curr.to + offset,
                        });
                        curr.from = curr.to + 1;
                    } else {
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: rule.source_start + rule.length - 1 + offset,
                        });
                        curr.from = rule.source_start + rule.length;
                    }
                }
            }
            if curr.from <= curr.to {
                new_ranges.push(curr);
            }
        }
        curr_ranges = new_ranges;
    }

    curr_ranges.iter().map(|range| range.from).min().unwrap()
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
        assert_eq!(result, 35);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 46);
    }
}
