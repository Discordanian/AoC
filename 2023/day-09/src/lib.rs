/*
lines = fin.read().strip().split("\n")


def diff(arr):
    return [arr[i+1] - arr[i] for i in range(len(arr) - 1)]


def extrapolate(hist):
    layers = [hist]

    while not all([x == 0 for x in layers[-1]]):
        layers.append(diff(layers[-1]))

    layers[-1].append(0)
    for i in range(len(layers) - 2, -1, -1):
        layers[i].append(layers[i][-1] + layers[i+1][-1])

    return layers[0][-1]


ans = []
for line in lines:
    arr = list(map(int, line.split()))
    ans.append(extrapolate(arr))

sum(ans)
 */
pub fn process_part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .iter()
        .map(|line| {
            let arr: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.trim().parse().unwrap())
                .collect();
            next1_value(arr)
        })
        .sum()
}

pub fn next1_value(arr: Vec<i32>) -> i32 {
    let mut matrix: Vec<Vec<i32>> = vec![arr];
    dbg!(&matrix);

    *matrix[0].last().unwrap()
}

/*
lines = fin.read().strip().split("\n")


def diff(arr):
    return [arr[i+1] - arr[i] for i in range(len(arr) - 1)]


def extrapolate(hist):
    layers = [hist]

    while not all([x == 0 for x in layers[-1]]):
        layers.append(diff(layers[-1]))

    layers[-1].append(0)
    for i in range(len(layers) - 2, -1, -1):
        layers[i].append(layers[i][-1] + layers[i+1][-1])

    return layers[0][-1]


ans = []
for line in lines:
    arr = list(map(int, line.split()[::-1]))
    ans.append(extrapolate(arr))

sum(ans)
*/

pub fn process_part2(input: &str) -> i32 {
    input.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 114);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 2);
    }
}
