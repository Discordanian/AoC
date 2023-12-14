pub fn process_part1(input: &str) -> u32 {
    let instructions: Vec<&str> = input.split(", ").collect();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    instructions.len() as u32
}

pub fn process_part2(input: &str) -> u32 {
    let instructions: Vec<&str> = input.split(", ").collect();
    instructions.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = process_part1("R2, L3");
        assert_eq!(result, 5);
    }

    #[test]
    fn part2_works() {
        let result = process_part2("R2, R2, R2");
        assert_eq!(result, 2);
    }
}
