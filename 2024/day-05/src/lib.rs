pub fn sorted_update(rules: &[Vec<i32>], update: &[i32]) -> Vec<i32> {
    let mut retval = vec![];
    let mut remaining = update.to_vec();

    while retval.len() != update.len() {
        let mut taken = false;
        for idx in 0..remaining.len() {
            if !taken {
                let x = remaining[idx];
                if rules[x as usize].iter().all(|z| !remaining.contains(z)) {
                    retval.push(x);
                    remaining.remove(idx);
                    taken = true;
                }
            }
        }
    }

    retval
}

pub fn good_update(rules: &[Vec<i32>], update: &[i32]) -> bool {
    let mut illegal: Vec<i32> = Vec::new();

    for v in update.iter() {
        if illegal.contains(v) {
            return false;
        }
        for x in rules[*v as usize].iter() {
            illegal.push(*x);
        }
    }

    true
}

pub fn middle_of_update(a: &[i32]) -> i32 {
    if a.is_empty() {
        return 0;
    }
    let mid = a.len() / 2;
    a[mid]
}
pub fn parse_vec_i32(s: &str) -> Vec<i32> {
    use regex::Regex;
    let re = Regex::new(r"(-?\d+)").expect("parse_vec_i32 regex failure");
    let mut retval = vec![];

    for (_, [x]) in re.captures_iter(s).map(|c| c.extract()) {
        retval.push(x.parse::<i32>().unwrap());
    }

    retval
}

pub fn process_part1(input: &str) -> i32 {
    let mut sections = input.split("\n\n");
    let rules_str = sections.next().unwrap();
    let updates_str = sections.next().unwrap();

    let mut rules: Vec<Vec<i32>> = vec![];
    for _ in 0..=99 {
        rules.push(vec![]);
    }

    for rule_line in rules_str.lines() {
        let mut i = rule_line.split("|");
        let first: i32 = i.next().unwrap().parse().unwrap();
        let second: usize = i.next().unwrap().parse().unwrap();

        rules[second].push(first);
    }

    let updates: Vec<Vec<i32>> = updates_str.lines().map(parse_vec_i32).collect();

    let mut ans = 0;
    for update in updates.iter() {
        if good_update(&rules, update) {
            ans += middle_of_update(update);
        }
    }
    ans
}

pub fn process_part2(input: &str) -> i32 {
    let mut sections = input.split("\n\n");
    let rules_str = sections.next().unwrap();
    let updates_str = sections.next().unwrap();

    let mut rules: Vec<Vec<i32>> = vec![];
    for _ in 0..=99 {
        rules.push(vec![]);
    }

    for rule_line in rules_str.lines() {
        let mut i = rule_line.split("|");
        let first: i32 = i.next().unwrap().parse().unwrap();
        let second: usize = i.next().unwrap().parse().unwrap();

        rules[second].push(first);
    }

    let updates: Vec<Vec<i32>> = updates_str.lines().map(parse_vec_i32).collect();

    let mut ans = 0;
    for update in updates.iter() {
        if !good_update(&rules, update) {
            // todo()! re-order update
            ans += middle_of_update(&sorted_update(&rules, update));
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 143);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 123);
    }
}
