pub fn process_part1(input: &str) -> u32 {
    input.len() as u32
}

pub fn process_part2(input: &str) -> u32 {
    input.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1a_works() {
        let result = process_part1("12");
        assert_eq!(result, 2);
    }

    #[test]
    fn part1b_works() {
        let result = process_part1("14");
        assert_eq!(result, 2);
    }

    #[test]
    fn part1c_works() {
        let result = process_part1("1969");
        assert_eq!(result, 654);
    }

    #[test]
    fn part1d_works() {
        let result = process_part1("100756");
        assert_eq!(result, 33583);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 15);
    }
}
