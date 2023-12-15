pub fn hash1(input: &str) -> u32 {
    input.trim_end().chars().fold(0_u32, |mut total, c| {
        total += c as u32;
        total *= 17;
        total % 256
    })
}

pub fn process_part1(input: &str) -> u32 {
    input.split(",").map(hash1).sum()
}

pub fn process_part2(input: &str) -> u32 {
    input.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 1320);
    }

    #[test]
    fn part1a_works() {
        let result = hash1("rn=1");
        assert_eq!(result, 30);
    }

    #[test]
    fn part1b_works() {
        let result = hash1("cm-");
        assert_eq!(result, 253);
    }

    /*
    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 15);
    }
    */
}
