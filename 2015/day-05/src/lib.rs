pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .filter(|line| is_nice_part1(line))
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .filter(|line| is_nice_part2(line))
        .count()
        .to_string()
}

fn is_nice_part1(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_double = false;
    let mut has_forbidden = false;
    let mut previous = None;

    for current in s.chars() {
        if matches!(current, 'a' | 'e' | 'i' | 'o' | 'u') {
            vowel_count += 1;
        }

        if let Some(prev) = previous {
            if prev == current {
                has_double = true;
            }

            if matches!((prev, current), ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')) {
                has_forbidden = true;
            }
        }

        previous = Some(current);
    }

    vowel_count >= 3 && has_double && !has_forbidden
}

fn is_nice_part2(s: &str) -> bool {
    let bytes = s.as_bytes();

    if bytes.len() < 3 {
        return false;
    }

    let mut has_repeated_pair = false;
    let mut first_seen = std::collections::HashMap::new();

    for i in 0..(bytes.len() - 1) {
        let pair = (bytes[i], bytes[i + 1]);

        if let Some(&first_index) = first_seen.get(&pair) {
            if i >= first_index + 2 {
                has_repeated_pair = true;
                break;
            }
        } else {
            first_seen.insert(pair, i);
        }
    }

    let has_repeat_with_gap = (0..(bytes.len() - 2)).any(|i| bytes[i] == bytes[i + 2]);

    has_repeated_pair && has_repeat_with_gap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let input = [
            "ugknbfddgicrmopn",
            "aaa",
            "jchzalrnumimnmhp",
            "haegwjzuvuyypxyu",
            "dvszwmarrgswjxmb",
        ]
        .join("\n");

        assert_eq!(process_part1(&input), "2");
    }

    #[test]
    fn helper_classifies_single_strings() {
        assert!(is_nice_part1("ugknbfddgicrmopn"));
        assert!(is_nice_part1("aaa"));
        assert!(!is_nice_part1("jchzalrnumimnmhp"));
        assert!(!is_nice_part1("haegwjzuvuyypxyu"));
        assert!(!is_nice_part1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part2_examples() {
        let input = [
            "qjhvhtzxzqqjkmpb",
            "xxyxx",
            "uurcxstgmygtbstg",
            "ieodomkazucvgmuy",
        ]
        .join("\n");

        assert_eq!(process_part2(&input), "2");
    }

    #[test]
    fn helper_part2_classifies_single_strings() {
        assert!(is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_part2("xxyxx"));
        assert!(!is_nice_part2("uurcxstgmygtbstg"));
        assert!(!is_nice_part2("ieodomkazucvgmuy"));
    }
}
