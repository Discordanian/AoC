use std::cmp::Ordering;

const TARGET_LITERS: u32 = 150;

fn parse_capacities(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

pub fn process_part1(input: &str) -> String {
    let capacities = parse_capacities(input);
    count_subsets_sum_to(&capacities, TARGET_LITERS).to_string()
}

fn count_subsets_sum_to(capacities: &[u32], target: u32) -> usize {
    fn dfs(i: usize, sum: u32, caps: &[u32], target: u32) -> usize {
        if sum == target {
            return 1;
        }
        if i == caps.len() || sum > target {
            return 0;
        }
        dfs(i + 1, sum, caps, target) + dfs(i + 1, sum + caps[i], caps, target)
    }

    dfs(0, 0, capacities, target)
}

pub fn process_part2(input: &str) -> String {
    let capacities = parse_capacities(input);
    count_ways_at_min_container_count(&capacities, TARGET_LITERS).to_string()
}

/// Among all subsets summing to `target`, count how many use the smallest possible number of elements.
fn count_ways_at_min_container_count(capacities: &[u32], target: u32) -> usize {
    fn merge(a: (Option<usize>, usize), b: (Option<usize>, usize)) -> (Option<usize>, usize) {
        match (a.0, b.0) {
            (None, None) => (None, 0),
            (Some(m1), None) => (Some(m1), a.1),
            (None, Some(m2)) => (Some(m2), b.1),
            (Some(m1), Some(m2)) => match m1.cmp(&m2) {
                Ordering::Less => (Some(m1), a.1),
                Ordering::Greater => (Some(m2), b.1),
                Ordering::Equal => (Some(m1), a.1 + b.1),
            },
        }
    }

    fn dfs(i: usize, sum: u32, used: usize, caps: &[u32], target: u32) -> (Option<usize>, usize) {
        if sum == target {
            return (Some(used), 1);
        }
        if i == caps.len() || sum > target {
            return (None, 0);
        }
        merge(
            dfs(i + 1, sum, used, caps, target),
            dfs(i + 1, sum + caps[i], used + 1, caps, target),
        )
    }

    dfs(0, 0, 0, capacities, target).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_from_puzzle() {
        assert_eq!(count_subsets_sum_to(&[20, 15, 10, 5, 5], 25), 4);
    }

    #[test]
    fn example_part2_from_puzzle() {
        assert_eq!(count_ways_at_min_container_count(&[20, 15, 10, 5, 5], 25), 3);
    }
}
