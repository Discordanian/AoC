use std::collections::HashMap;

pub fn design_possible(
    cache: &mut HashMap<String, bool>,
    patterns: Vec<String>,
    design: String,
) -> bool {
    // Is it trivial
    if design.len() == 0 {
        return true;
    }
    // Is it cached
    if let Some(result) = cache.get(&design) {
        return *result;
    }
    // Is it trivial
    if design.len() == 1 && patterns.contains(&design) {
        cache.insert(design, true);
        return true;
    }

    let design_l = design.len();

    for i in 0..design_l {
        let firstpart = design[0..i].to_string();
        let secondpart = design[i..design_l].to_string();
        if patterns.contains(&firstpart) && design_possible(cache, patterns.clone(), secondpart) {
            cache.insert(design, true);
            return true;
        }
    }

    cache.insert(design, false);
    false
}

pub fn process_part1(input: &str) -> usize {
    let mut lines = input.lines();

    let patterns = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    lines.next();
    let mut designs: Vec<String> = Vec::new();
    // while let Some(d) = lines.next() {
    for design in lines {
        designs.push(design.to_string());
    }

    let mut cache: HashMap<String, bool> = HashMap::new();

    let x = designs
        .into_iter()
        .filter(|x| design_possible(&mut cache, patterns.clone(), x.to_string()))
        .count();
    // dbg!(cache);
    x
}

pub fn process_part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 6);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 0);
    }
}
