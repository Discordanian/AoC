use std::collections::HashMap;

pub fn design_possible(
    cache: &mut HashMap<String, bool>,
    patterns: Vec<String>,
    design: String,
) -> bool {
    if let Some(result) = cache.get(&design) {
        return *result;
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
    // Set up trivial cache
    cache.insert(String::from(""), true); // Insert default case
    for p in patterns.clone().into_iter() {
        cache.insert(p.clone(), true);
    }

    designs
        .into_iter()
        .filter(|x| design_possible(&mut cache, patterns.clone(), x.to_string()))
        .count()
}

pub fn designs_possible(
    cache: &mut HashMap<String, u64>,
    patterns: Vec<String>,
    design: String,
) -> u64 {
    // Let the cache do its magic
    if let Some(result) = cache.get(&design) {
        return *result;
    }
    let mut retval = 0;

    let design_l = design.len();

    for i in 0..design_l {
        let firstpart = design[0..i].to_string();
        let secondpart = design[i..design_l].to_string();
        if patterns.contains(&firstpart) {
            dbg!(("before", firstpart.clone(), secondpart.clone(), retval));
            retval += designs_possible(cache, patterns.clone(), secondpart.clone());
            dbg!(("after", firstpart.clone(), secondpart.clone(), retval));
        }
    }
    /*
    @cache
    def designs_possible(design):
        if design == "": return 1
        count = 0
        for i in range(len(design)):
            if design[:i] in patterns:
                count += designs_possible(design[i:])
        return count

    print(sum(num_possibilities(design) for design in lines[2:]))
        */
    cache.insert(design.clone(), retval);
    retval
}

pub fn process_part2(input: &str) -> u64 {
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

    let mut cache: HashMap<String, u64> = HashMap::new();
    // Set up trivial cache
    cache.insert(String::from(""), 1); // Insert default case
    for p in patterns.iter() {
        if p.len() == 1 {
            cache.insert(p.clone(), 1);
        }
    }
    dbg!(&cache);

    let x = designs
        .into_iter()
        .map(|x| designs_possible(&mut cache, patterns.clone(), x.to_string()))
        .sum();
    dbg!(&cache);
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALLINPUT: &str = "r, wr, b, g, bwu, rb, gb, br

gbbr";

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
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 16);
    }

    #[test]
    fn part2_small() {
        let result = process_part2(SMALLINPUT);
        assert_eq!(result, 4);
    }
}
