use std::collections::HashMap;

fn mfcsam_readings() -> HashMap<&'static str, u32> {
    [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .collect()
}

fn compound_matches_part2(key: &str, value: u32, tape: &HashMap<&'static str, u32>) -> bool {
    let Some(&expected) = tape.get(key) else {
        return false;
    };
    match key {
        "cats" | "trees" => value > expected,
        "pomeranians" | "goldfish" => value < expected,
        _ => value == expected,
    }
}

fn matching_sue(line: &str, tape: &HashMap<&'static str, u32>, part2_ranges: bool) -> Option<u32> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    let (head, props) = line.split_once(':')?;
    let sue: u32 = head.strip_prefix("Sue ")?.parse().ok()?;
    for segment in props.split(',') {
        let segment = segment.trim();
        let (key, raw) = segment.split_once(':')?;
        let key = key.trim();
        let value: u32 = raw.trim().parse().ok()?;
        let ok = if part2_ranges {
            compound_matches_part2(key, value, tape)
        } else {
            tape.get(key).copied() == Some(value)
        };
        if !ok {
            return None;
        }
    }
    Some(sue)
}

pub fn process_part1(input: &str) -> String {
    let tape = mfcsam_readings();
    for line in input.lines() {
        if let Some(sue) = matching_sue(line, &tape, false) {
            return sue.to_string();
        }
    }
    panic!("no Aunt Sue matched the MFCSAM tape");
}

pub fn process_part2(input: &str) -> String {
    let tape = mfcsam_readings();
    for line in input.lines() {
        if let Some(sue) = matching_sue(line, &tape, true) {
            return sue.to_string();
        }
    }
    panic!("no Aunt Sue matched the MFCSAM tape (range rules)");
}

#[cfg(test)]
mod tests {}
