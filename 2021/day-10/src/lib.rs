pub fn score_bracket(bracket: char) -> u32 {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("What bracket is this?"),
    }
}

pub fn line_score2(line: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();
    let mut retval = 0;

    for bracket in line.chars() {
        if matches!(bracket, '(' | '{' | '<' | '[') {
            match bracket {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                '[' => stack.push(']'),
                _ => panic!("Should be an open bracket"),
            }
        } else if Some(bracket) != stack.pop() {
            return 0;
        }
    }

    while let Some(bracket) = stack.pop() {
        retval *= 5;
        retval += match bracket {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("What bracket is this?"),
        }
    }

    retval
}
pub fn line_score1(line: &str) -> u32 {
    let mut stack: Vec<char> = Vec::new();

    for bracket in line.chars() {
        if matches!(bracket, '(' | '{' | '<' | '[') {
            match bracket {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                '[' => stack.push(']'),
                _ => panic!("Should be an open bracket"),
            }
        } else if Some(bracket) != stack.pop() {
            return score_bracket(bracket);
        }
    }
    0
}
pub fn process_part1(input: &str) -> u32 {
    input.lines().map(line_score1).sum()
}

pub fn process_part2(input: &str) -> u64 {
    let mut scores = input
        .lines()
        .map(line_score2)
        .filter(|x| *x > 0)
        .collect::<Vec<u64>>();

    scores.sort();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1_example() {
        let result = process_part1(INPUT);
        assert_eq!(result, 26397);
    }

    #[test]
    fn part2_example() {
        let result = process_part2(INPUT);
        assert_eq!(result, 288957);
    }
}
