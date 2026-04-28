pub fn process_part1(input: &str) -> String {
    let secret_key = input.trim();

    for n in 1u64.. {
        let candidate = format!("{secret_key}{n}");
        let digest = md5::compute(candidate);

        if format!("{digest:x}").starts_with("00000") {
            return n.to_string();
        }
    }

    unreachable!("The search space is unbounded and should always find a solution.")
}

pub fn process_part2(input: &str) -> String {
    let secret_key = input.trim();

    for n in 1u64.. {
        let candidate = format!("{secret_key}{n}");
        let digest = md5::compute(candidate);

        if format!("{digest:x}").starts_with("000000") {
            return n.to_string();
        }
    }

    unreachable!("The search space is unbounded and should always find a solution.")
}
