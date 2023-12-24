/*

with open("./input.txt") as fin:
    lines = fin.read().strip().split("\n")

ss = []
spring_configurations = []
for line in lines:
    parts = line.split(" ")
    ss.append(parts[0])
    spring_configurations.append(list(map(int, parts[1].split(","))))


def valid(line, spring_configurations):
    n = len(line)
    runs = []

    i = 0
    while i < n:
        while i < n and not line[i]:
            i += 1
        if i == n:
            break
        j = i
        c = 0
        while j < n and line[j]:
            j += 1
            c += 1

        runs.append(c)
        i = j

    return runs == spring_configurations


def ways(s, spring_configurations):
    line = []
    idxs = []
    for i, x in enumerate(s):
        if x == ".":
            line.append(0)
        if x == "?":
            line.append(-1)
            idxs.append(i)
        if x == "#":
            line.append(1)

    count = 0
    for mask in range(1 << len(idxs)):
        line_copy = line.copy()
        for i in range(len(idxs)):
            if mask & (1 << i):
                line_copy[idxs[i]] = 0
            else:
                line_copy[idxs[i]] = 1

        if valid(line_copy, spring_configurations):
            count += 1

    return count


ans = 0
for s, target_run in list(zip(ss, spring_configurations)):
    res = ways(s, target_run)
    ans += res

print(ans)
*/
/*

from pprint import pprint

with open("./input.txt") as fin:
    lines = fin.read().strip().split("\n")

ss = []
spring_configurations = []
for line in lines:
    parts = line.split(" ")
    parts[0] = "?".join([parts[0]] * 5)
    parts[1] = ",".join([parts[1]] * 5)

    ss.append(parts[0])
    spring_configurations.append(list(map(int, parts[1].split(","))))


def ways(s, spring_configurations):
    # 3D dp on (idx in string, idx in set, length of current run)
    # Question: How many ways?
    spring_configurations.append(0)
    max_run = max(spring_configurations)
    s += "."

    n = len(s)
    m = len(spring_configurations)
    dp = [[[None for _ in range(max_run+1)]
           for _ in range(m)] for _ in range(n)]

    for i in range(n):
        x = s[i]
        for j in range(m):
            for k in range(spring_configurations[j]+1):
                # Base case
                if i == 0:
                    if j != 0:
                        dp[i][j][k] = 0
                        continue

                    if x == "#":
                        if k != 1:
                            dp[i][j][k] = 0
                            continue
                        dp[i][j][k] = 1
                        continue

                    if x == ".":
                        if k != 0:
                            dp[i][j][k] = 0
                            continue
                        dp[i][j][k] = 1
                        continue

                    if x == "?":
                        if k not in [0, 1]:
                            dp[i][j][k] = 0
                            continue
                        dp[i][j][k] = 1
                        continue

                # Process answer if current char is .
                if k != 0:
                    ans_dot = 0
                elif j > 0:
                    assert k == 0
                    ans_dot = dp[i-1][j-1][spring_configurations[j-1]]
                    ans_dot += dp[i-1][j][0]
                else:
                    # i>0, j=0, k=0.
                    # Only way to do this is if every ? is a .
                    ans_dot = 1 if s[:i].count("#") == 0 else 0

                # Process answer if current char is #
                if k == 0:
                    ans_pound = 0
                else:
                    # Newest set
                    ans_pound = dp[i-1][j][k-1]

                if x == ".":
                    dp[i][j][k] = ans_dot
                elif x == "#":
                    dp[i][j][k] = ans_pound
                else:
                    dp[i][j][k] = ans_dot + ans_pound

    ans = dp[n-1][-1][0]

    return ans


ans = 0
for s, target_run in list(zip(ss, spring_configurations))
    res = ways(s, target_run)
    ans += res

print(ans)
*/

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

fn parse(input: &str) -> impl Iterator<Item = (Vec<Spring>, Vec<usize>)> + '_ {
    input.lines().map(|line| {
        let (springs, counts) = line.split_once(' ').unwrap();
        let springs: Vec<Spring> = springs
            .chars()
            .map(|c| match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("at the disco"),
            })
            .collect();
        let counts: Vec<usize> = counts.split(',').filter_map(|s| s.parse().ok()).collect();

        (springs, counts)
    })
}

pub fn process_part1(input: &str) -> u64 {
    parse(input)
        .map(|(springs, counts)| count_possible_arangements(springs, counts))
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    parse(input)
        .map(|(mut springs, mut counts)| {
            springs = springs
                .iter()
                .copied()
                .chain([Spring::Unknown])
                .cycle()
                .take(springs.len() * 5 + 4)
                .collect();
            counts = counts
                .iter()
                .copied()
                .cycle()
                .take(counts.len() * 5)
                .collect();

            count_possible_arangements(springs, counts)
        })
        .sum()
}

fn count_possible_arangements(mut springs: Vec<Spring>, counts: Vec<usize>) -> u64 {
    // to make the Damaged recursion case simpler
    springs.push(Spring::Operational);
    let mut cache = vec![vec![None; springs.len()]; counts.len()];
    count_possible_arangements_inner(&springs, &counts, &mut cache)
}

fn count_possible_arangements_inner(
    springs: &[Spring],
    counts: &[usize],
    cache: &mut [Vec<Option<u64>>],
) -> u64 {
    if counts.is_empty() {
        return if springs.contains(&Spring::Damaged) {
            // Too many previous unknowns were counted as damaged
            0
        } else {
            // All remaining unknowns are operational
            1
        };
    }
    if springs.len() < counts.iter().sum::<usize>() + counts.len() {
        // Not enough space for remaining numbers
        return 0;
    }
    if let Some(cached) = cache[counts.len() - 1][springs.len() - 1] {
        return cached;
    }
    let mut arangements = 0;
    if springs[0] != Spring::Damaged {
        // Assume operational
        arangements += count_possible_arangements_inner(&springs[1..], counts, cache);
    }
    let next_group_size = counts[0];
    if !springs[..next_group_size].contains(&Spring::Operational)
        && springs[next_group_size] != Spring::Damaged
    {
        // Assume damaged
        arangements +=
            count_possible_arangements_inner(&springs[next_group_size + 1..], &counts[1..], cache);
    }
    cache[counts.len() - 1][springs.len() - 1] = Some(arangements);
    arangements
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 525152);
    }
}
