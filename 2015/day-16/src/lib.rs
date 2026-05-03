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

pub fn process_part1(input: &str) -> String {
    let tape = mfcsam_readings();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (head, props) = line.split_once(':').expect("Sue N:");
        let sue: u32 = head
            .strip_prefix("Sue ")
            .expect("Sue prefix")
            .parse()
            .expect("Sue number");
        let mut matches = true;
        for segment in props.split(',') {
            let segment = segment.trim();
            let (key, raw) = segment.split_once(':').expect("key: value");
            let key = key.trim();
            let value: u32 = raw.trim().parse().expect("compound count");
            if tape.get(key).copied() != Some(value) {
                matches = false;
                break;
            }
        }
        if matches {
            return sue.to_string();
        }
    }
    panic!("no Aunt Sue matched the MFCSAM tape");
}

pub fn process_part2(input: &str) -> String {
    input.len().to_string()
}

#[cfg(test)]
mod tests {}
