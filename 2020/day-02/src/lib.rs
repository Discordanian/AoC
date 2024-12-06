#[derive(Clone, Debug)]
pub struct Policy {
    min: i32,
    max: i32,
    character: char,
    password: Vec<char>,
}

impl From<&str> for Policy {
    fn from(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect(); // A-B C: D
                                                                  /*
                                                                  let counts: Vec<i32> = parts[0].split("-").map(|x| x.parse::<i32>()).collect();
                                                                  let character: char = parts[1].chars().nth(0).unwrap();

                                                                  Self {
                                                                      min: counts[0],
                                                                      max: counts[1],
                                                                      character: character,
                                                                      password: parts[2].chars().collect(),
                                                                  } */
        Self {
            min: 0,
            max: 0,
            character: 'X',
            password: parts[2].chars().collect(),
        }
    }
}

pub fn process_part1(input: &str) -> i32 {
    let policies: Vec<Policy> = input.lines().map(|x| Policy::from(x)).collect();
    policies.len() as i32
}

pub fn process_part2(input: &str) -> i32 {
    let policies: Vec<Policy> = input.lines().map(|x| Policy::from(x)).collect();
    policies.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "21-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 15);
    }
}
