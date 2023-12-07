pub fn process_part1(input: &str) -> String {
    let up: i32 = input.chars().filter(|x| *x == '(').count() as i32;
    let down: i32 = input.chars().filter(|x| *x == ')').count() as i32;
    (up - down).to_string()

}

pub fn process_part2(input: &str) -> String {
    let mut i = 1;
    let mut floor = 0;
    for c in input.chars() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Unknown Char"),
        };
        if floor == -1 {
            return i.to_string();
        }
        i+=1;
    }
    0.to_string()
}
