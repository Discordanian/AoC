
enum Direction {
    North,
    East,
    South,
    West
};

pub fn turn(d: Direction) {

}

pub fn process_part1(input: &str) -> u32 {
    let mut pos: (i32,i32) = input.split(", ").iter().fold((0_i32,0_i32, Direction::North) |())
    pos.0 + pos.1
}

pub fn process_part2(input: &str) -> u32 {
    input.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn part1_works() {
        let result = process_part1("R2, L3");
        assert_eq!(result, 5);
        let result = process_part1("R2, R2, R2");
        assert_eq!(result, 2);
        let result = process_part1("R5, L5, R5, R3");
        assert_eq!(result, 12);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 0);
    }
}
