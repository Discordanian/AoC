pub fn process_part1(input: &str) -> String {
    let total: u32 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let dims: Vec<u32> = line
                .split('x')
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect();

            let l = dims[0];
            let w = dims[1];
            let h = dims[2];

            let side1 = l * w;
            let side2 = w * h;
            let side3 = h * l;
            let slack = side1.min(side2).min(side3);

            2 * side1 + 2 * side2 + 2 * side3 + slack
        })
        .sum();

    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let total: u32 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut dims: Vec<u32> = line
                .split('x')
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect();
            dims.sort_unstable();

            let wrap = 2 * (dims[0] + dims[1]);
            let bow = dims[0] * dims[1] * dims[2];

            wrap + bow
        })
        .sum();

    total.to_string()
}
