use std::collections::{HashSet, VecDeque};

/// Split a molecule string into element symbols (uppercase, optional following lowercase).
fn token_spans(s: &str) -> Vec<(usize, usize)> {
    let b = s.as_bytes();
    let mut spans = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let start = i;
        i += 1;
        if i < b.len() && b[i].is_ascii_lowercase() {
            i += 1;
        }
        spans.push((start, i));
    }
    spans
}

fn parse_input(input: &str) -> (Vec<(String, String)>, String) {
    let mut rules = Vec::new();
    let mut medicine = String::new();
    for line in input.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        if let Some((from, to)) = line.split_once(" => ") {
            rules.push((from.to_string(), to.to_string()));
        } else {
            medicine = line.to_string();
        }
    }
    (rules, medicine)
}

/// Count distinct molecules reachable with exactly one replacement.
pub fn process_part1(input: &str) -> String {
    let (rules, medicine) = parse_input(input);
    let spans = token_spans(&medicine);
    let tokens: Vec<&str> = spans
        .iter()
        .map(|&(s, e)| &medicine[s..e])
        .collect();

    let mut seen = HashSet::new();
    for (from, to) in &rules {
        let from_spans = token_spans(from);
        let from_len = from_spans.len();
        if from_len == 0 || from_len > tokens.len() {
            continue;
        }
        for i in 0..=tokens.len() - from_len {
            let mut ok = true;
            for j in 0..from_len {
                if tokens[i + j] != &from[from_spans[j].0..from_spans[j].1] {
                    ok = false;
                    break;
                }
            }
            if !ok {
                continue;
            }
            let start = spans[i].0;
            let end = spans[i + from_len - 1].1;
            let mut next = String::with_capacity(medicine.len() - (end - start) + to.len());
            next.push_str(&medicine[..start]);
            next.push_str(to);
            next.push_str(&medicine[end..]);
            seen.insert(next);
        }
    }
    seen.len().to_string()
}

fn count_substrings(haystack: &str, needle: &str) -> usize {
    haystack.match_indices(needle).count()
}

/// Fewest reverse steps from `medicine` down to `e` (used for small / example inputs).
fn part2_bfs(rules: &[(String, String)], medicine: &str) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((medicine.to_string(), 0usize));
    seen.insert(medicine.to_string());

    while let Some((cur, dist)) = queue.pop_front() {
        if cur == "e" {
            return dist;
        }
        for (lhs, rhs) in rules {
            let mut start = 0usize;
            while let Some(rel) = cur[start..].find(rhs) {
                let i = start + rel;
                let mut next = String::with_capacity(cur.len() - rhs.len() + lhs.len());
                next.push_str(&cur[..i]);
                next.push_str(lhs);
                next.push_str(&cur[i + rhs.len()..]);
                if seen.insert(next.clone()) {
                    queue.push_back((next, dist + 1));
                }
                start = i + 1;
            }
        }
    }
    panic!("no path from medicine to e");
}

/// Fewest forward steps from `e` to the medicine molecule.
pub fn process_part2(input: &str) -> String {
    let (rules, medicine) = parse_input(input);
    // Official inputs use Rn / Ar / Y as grammar markers; the askalski shortcut applies.
    // Example inputs from the statement have no Rn; use exact BFS on the reverse graph.
    if medicine.contains("Rn") {
        let tokens = token_spans(&medicine).len();
        let rn = count_substrings(&medicine, "Rn");
        let ar = count_substrings(&medicine, "Ar");
        let y = count_substrings(&medicine, "Y");
        (tokens - rn - ar - 2 * y - 1).to_string()
    } else {
        part2_bfs(&rules, &medicine).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "H => HO
H => OH
O => HH

HOH";

    #[test]
    fn part1_example_hoh() {
        assert_eq!(process_part1(EXAMPLE), "4");
    }

    #[test]
    fn part1_example_hohoho() {
        let input = "H => HO
H => OH
O => HH

HOHOHO";
        assert_eq!(process_part1(input), "7");
    }

    const EXAMPLE_P2: &str = "e => H
e => O
H => HO
H => OH
O => HH

HOH";

    #[test]
    fn part2_example_hoh() {
        assert_eq!(process_part2(EXAMPLE_P2), "3");
    }

    #[test]
    fn part2_example_hohoho() {
        let input = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO";
        assert_eq!(process_part2(input), "6");
    }

    #[test]
    fn part2_puzzle_input() {
        let input = include_str!("../input.txt");
        assert_eq!(process_part2(input), "207");
    }
}
