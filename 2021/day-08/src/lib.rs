use itertools::Itertools;

pub fn process_part1(input: &str) -> usize {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(" | ").collect::<Vec<&str>>())
        .map(|v| v[1])
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| l.split_ascii_whitespace().collect())
        .collect();
    lines
        .into_iter()
        .map(|v| {
            v.into_iter()
                .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                .count()
        })
        .sum()
}

pub fn solve_line(s: &str) -> u32 {
    let perms: Vec<String> = "abcdefg"
        .chars()
        .collect::<Vec<char>>()
        .into_iter()
        .permutations(7)
        .map(|x| x.iter().collect::<String>())
        .collect();

    perms.len() as u32
}

pub fn process_part2(input: &str) -> u32 {
    input.lines().map(solve_line).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    const SMALLINPUT: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf";

    #[test]
    fn part1_example() {
        let result = process_part1(INPUT);
        assert_eq!(result, 26);
    }

    #[test]
    fn part2a_example() {
        let result = process_part2(SMALLINPUT);
        assert_eq!(result, 5353);
    }

    #[test]
    #[ignore]
    fn part2b_example() {
        let result = process_part2(INPUT);
        assert_eq!(result, 61229);
    }
}
