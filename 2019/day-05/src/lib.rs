mod computer;
use crate::computer::Computer;
pub fn process_part1(input: &str) -> i32 {
    let mut computer: Computer = Computer::new_with_input(input, &[1]);
    computer.execute();

    computer.obuff.pop().expect("Output Buffer has value")
}

pub fn process_part2(input: &str) -> i32 {
    let mut computer: Computer = Computer::new_with_input(input, &[5]);
    computer.execute();

    computer.obuff.pop().expect("Output Buffer has value")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1002,4,3,4,33";

    #[test]
    #[ignore]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 10);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 0);
    }
}
