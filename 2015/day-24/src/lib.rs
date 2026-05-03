pub fn process_part1(input: &str) -> String {
    let weights = parse_weights(input);
    let total: u64 = weights.iter().sum();
    let target = total / 3;

    for k in 1..=weights.len() {
        if let Some(best_qe) = min_quantum_entanglement(&weights, target, k, |rem| {
            debug_assert_eq!(rem.iter().sum::<u64>(), 2 * target);
            subset_sums_to(rem, target)
        }) {
            return best_qe.to_string();
        }
    }

    "0".to_string()
}

pub fn process_part2(input: &str) -> String {
    let weights = parse_weights(input);
    let total: u64 = weights.iter().sum();
    let target = total / 4;

    for k in 1..=weights.len() {
        if let Some(best_qe) = min_quantum_entanglement(&weights, target, k, |rem| {
            debug_assert_eq!(rem.iter().sum::<u64>(), 3 * target);
            can_partition_into_k_groups(rem, 3, target)
        }) {
            return best_qe.to_string();
        }
    }

    "0".to_string()
}

fn parse_weights(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

/// Whether some subset of `items` sums exactly to `target`.
fn subset_sums_to(items: &[u64], target: u64) -> bool {
    let t = target as usize;
    let mut dp = vec![false; t + 1];
    dp[0] = true;
    for &w in items {
        let w = w as usize;
        if w > t {
            continue;
        }
        for s in (w..=t).rev() {
            if dp[s - w] {
                dp[s] = true;
            }
        }
    }
    dp[t]
}

/// Whether `items` can be split into `k` disjoint groups, each summing to `target`.
fn can_partition_into_k_groups(items: &[u64], k: usize, target: u64) -> bool {
    if k == 0 {
        return items.is_empty();
    }
    if k == 1 {
        return items.iter().sum::<u64>() == target;
    }

    let sum: u64 = items.iter().sum();
    if sum != k as u64 * target {
        return false;
    }
    if items.iter().any(|&w| w > target) {
        return false;
    }

    let mut sorted = items.to_vec();
    sorted.sort_by_key(|&w| std::cmp::Reverse(w));
    let mut buckets = vec![0u64; k];
    partition_dfs(&sorted, &mut buckets, target, 0)
}

fn partition_dfs(sorted: &[u64], buckets: &mut [u64], target: u64, idx: usize) -> bool {
    if idx == sorted.len() {
        return buckets.iter().all(|&b| b == target);
    }
    let w = sorted[idx];
    for i in 0..buckets.len() {
        if i > 0 && buckets[i] == buckets[i - 1] {
            continue;
        }
        if buckets[i] + w <= target {
            buckets[i] += w;
            if partition_dfs(sorted, buckets, target, idx + 1) {
                return true;
            }
            buckets[i] -= w;
        }
    }
    false
}

/// Items sorted by weight descending (index, weight) for search pruning.
fn sorted_by_weight_desc(weights: &[u64]) -> Vec<(usize, u64)> {
    let mut v: Vec<_> = weights.iter().copied().enumerate().collect();
    v.sort_by_key(|&(_, w)| std::cmp::Reverse(w));
    v
}

/// Among all `k`-subsets summing to `target`, return min quantum entanglement if any
/// passes `remainder_ok` on the remaining packages.
fn min_quantum_entanglement<F>(weights: &[u64], target: u64, k: usize, remainder_ok: F) -> Option<u128>
where
    F: Fn(&[u64]) -> bool,
{
    let items = sorted_by_weight_desc(weights);
    let mut best: Option<u128> = None;
    let mut picked = Vec::with_capacity(k);

    dfs_first_groups(
        &items,
        0,
        &mut picked,
        0,
        target,
        k,
        weights,
        &mut best,
        &remainder_ok,
    );
    best
}

fn dfs_first_groups<F: Fn(&[u64]) -> bool>(
    items: &[(usize, u64)],
    pos: usize,
    picked: &mut Vec<usize>,
    sum: u64,
    target: u64,
    k: usize,
    weights: &[u64],
    best: &mut Option<u128>,
    remainder_ok: &F,
) {
    if picked.len() == k {
        if sum == target {
            let mut rem = Vec::with_capacity(weights.len() - k);
            let mut in_first = vec![false; weights.len()];
            for &i in picked.iter() {
                in_first[i] = true;
            }
            for i in 0..weights.len() {
                if !in_first[i] {
                    rem.push(weights[i]);
                }
            }
            if remainder_ok(&rem) {
                let qe: u128 = picked.iter().map(|&i| weights[i] as u128).product();
                *best = Some(best.map_or(qe, |b| b.min(qe)));
            }
        }
        return;
    }

    let need = k - picked.len();
    for i in pos..items.len() {
        if items.len() - i < need {
            break;
        }
        let (idx, w) = items[i];
        if sum + w > target {
            continue;
        }
        picked.push(idx);
        dfs_first_groups(
            items,
            i + 1,
            picked,
            sum + w,
            target,
            k,
            weights,
            best,
            remainder_ok,
        );
        picked.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1
2
3
4
5
7
8
9
10
11
";

    #[test]
    fn example_part1() {
        assert_eq!(process_part1(EXAMPLE), "99");
    }

    #[test]
    fn example_part2() {
        assert_eq!(process_part2(EXAMPLE), "44");
    }
}
