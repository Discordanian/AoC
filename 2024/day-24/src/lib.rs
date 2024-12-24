use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Register {
    Value(u64),
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

pub fn make_input_map(s: &str) -> HashMap<String, Register> {
    let mut map: HashMap<String, Register> = HashMap::new();

    let sections: Vec<&str> = s.split("\n\n").collect();
    for line in sections[0].lines() {
        let line_split: Vec<&str> = line.split(": ").collect();
        let label = line_split[0].to_string();
        let val = line_split[1]
            .parse()
            .expect("Able to parse input value to register");
        map.insert(label, Register::Value(val));
    }
    for line in sections[1].lines() {
        let line_split: Vec<&str> = line.split_ascii_whitespace().collect();
        let label = line_split[4].to_string();
        let lhs = String::from(line_split[0]);
        let rhs = String::from(line_split[2]);
        match line_split[1] {
            "AND" => map.insert(label, Register::And(lhs, rhs)),
            "OR" => map.insert(label, Register::Or(lhs, rhs)),
            "XOR" => map.insert(label, Register::Xor(lhs, rhs)),
            _ => panic!("Unknown Operation type"),
        };
    }

    map
}

pub fn value_wire(map: &HashMap<String, Register>, label: &String) -> u64 {
    match map.get(label) {
        None => panic!("Label not found in map!"),
        Some(Register::Value(x)) => *x,
        Some(Register::And(a, b)) => value_wire(map, a) & value_wire(map, b),
        Some(Register::Or(a, b)) => value_wire(map, a) | value_wire(map, b),
        Some(Register::Xor(a, b)) => value_wire(map, a) ^ value_wire(map, b),
    }
}

pub fn make_wire(c: char, i: usize) -> String {
    format!("{}{:02}", c, i)
}

pub fn process_part1(input: &str, bits: usize) -> u64 {
    let map = make_input_map(input);
    let mut retval = 0;

    for i in 0..bits {
        let label: String = make_wire('z', i);
        retval += value_wire(&map, &label) << i;
    }

    retval
}

pub fn process_part2(input: &str, bits: usize, pairs: usize) -> String {
    let map = make_input_map(input);
    let mut retval = (bits + pairs) as u64;

    for i in 0..bits {
        let label: String = make_wire('z', i);
        retval += value_wire(&map, &label) << i;
    }

    retval.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALLINPUT: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    const PAIRS: usize = 2;

    #[test]
    fn part1a_works() {
        let result = process_part1(SMALLINPUT, 3);
        assert_eq!(result, 4);
    }

    #[test]
    fn part1b_works() {
        let result = process_part1(INPUT, 13);
        assert_eq!(result, 2024);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT, 13, PAIRS);
        assert_eq!(result, "".to_string());
    }
}
