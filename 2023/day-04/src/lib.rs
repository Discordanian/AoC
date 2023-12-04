use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Card {
    id: usize,
    left: BTreeSet<u32>,
    right: BTreeSet<u32>,
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let parts: Vec<&str> = line.split(": ").collect();
        let n: usize = parts[0][5..].trim().parse().expect("Unable to parse Card id");
        let numbers_part: Vec<&str> = parts[1].split("| ").collect();
        let l = numbers_part[0]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<BTreeSet<u32>>();
        let r = numbers_part[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<BTreeSet<u32>>();
        Card {
            id: n,
            left: l,
            right: r,
        }
    }
}

fn part1_score(score: u32) -> u32 {
    match score {
        0 => 0,
        1 => 1,
        _ => 2_u32.pow(score - 1),
    }
}

pub fn process_part1(input: &str) -> String {
    // let cards: Vec<Card> = input.lines().map(|c| Card::from(c)).collect();
    let cards: Vec<Card> = input.lines().map(Card::from).collect();
    let mut score = 0;

    for card in cards {
        let matches = card.right.iter().filter(|x| card.left.contains(x)).count();
        score += part1_score(matches as u32);
    }

    score.to_string()
}

pub fn process_part2(input: &str) -> String {
    let cards: Vec<Card> = input.lines().map(Card::from).collect();
    let size = cards.len() + 1;
    let mut scratchers: Vec<u32> = vec![1; size];
    scratchers[0] = 0;

    for card in cards {
        let matches = card.right.iter().filter(|x| card.left.contains(x)).count();
        for x in 1..=matches {
            let up_idx = card.id + x;
            scratchers[up_idx] += scratchers[card.id];
        }
        // score += part1_score(matches as u32);
    }


    scratchers.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT : &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "13".to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "30".to_string());
    }
}
