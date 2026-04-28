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
    use std::fmt::Write;

    let secret_key = input.trim();
    let mut candidate = String::with_capacity(secret_key.len() + 20);

    for n in 1u64.. {
        candidate.clear();
        candidate.push_str(secret_key);
        write!(&mut candidate, "{n}").expect("writing into a String cannot fail");
        let digest = md5::compute(candidate.as_bytes());

        if digest.0[0] == 0 && digest.0[1] == 0 && digest.0[2] == 0 {
            return n.to_string();
        }
    }

    unreachable!("The search space is unbounded and should always find a solution.")
}
