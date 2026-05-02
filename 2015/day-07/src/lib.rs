use std::collections::HashMap;

#[derive(Clone)]
enum Operand {
    Lit(u16),
    Wire(String),
}

#[derive(Clone)]
enum Expr {
    Signal(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, u8),
    RShift(Operand, u8),
    Not(Operand),
}

fn parse_operand(s: &str) -> Operand {
    let trimmed: &str = s.trim();
    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        Operand::Lit(trimmed.parse().unwrap())
    } else {
        Operand::Wire(trimmed.to_string())
    }
}

fn parse_line(line: &str) -> Option<(String, Expr)> {
    let trimmed: &str = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    let (lhs, rhs): (&str, &str) = trimmed.split_once(" -> ")?;
    let target: String = rhs.trim().to_string();
    let lhs: &str = lhs.trim();

    let expr: Expr = if let Some(rest) = lhs.strip_prefix("NOT ") {
        Expr::Not(parse_operand(rest))
    } else if let Some((a, b)) = lhs.split_once(" AND ") {
        Expr::And(parse_operand(a), parse_operand(b))
    } else if let Some((a, b)) = lhs.split_once(" OR ") {
        Expr::Or(parse_operand(a), parse_operand(b))
    } else if let Some((a, b)) = lhs.split_once(" LSHIFT ") {
        Expr::LShift(parse_operand(a), b.trim().parse().ok()?)
    } else if let Some((a, b)) = lhs.split_once(" RSHIFT ") {
        Expr::RShift(parse_operand(a), b.trim().parse().ok()?)
    } else {
        Expr::Signal(parse_operand(lhs))
    };

    Some((target, expr))
}

fn parse_circuit(input: &str) -> HashMap<String, Expr> {
    let mut circuit: HashMap<String, Expr> = HashMap::new();
    for line in input.lines() {
        let line: &str = line;
        if let Some(parsed) = parse_line(line) {
            let (target, expr): (String, Expr) = parsed;
            circuit.insert(target, expr);
        }
    }
    circuit
}

fn eval_operand(
    op: &Operand,
    circuit: &HashMap<String, Expr>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    match op {
        Operand::Lit(n) => *n,
        Operand::Wire(name) => eval_wire(name, circuit, cache),
    }
}

fn eval_wire(name: &str, circuit: &HashMap<String, Expr>, cache: &mut HashMap<String, u16>) -> u16 {
    if let Some(&v) = cache.get(name) {
        return v;
    }
    let expr: &Expr = circuit
        .get(name)
        .unwrap_or_else(|| panic!("unknown wire: {name}"));
    let v: u16 = match expr {
        Expr::Signal(o) => eval_operand(o, circuit, cache),
        Expr::And(a, b) => eval_operand(a, circuit, cache) & eval_operand(b, circuit, cache),
        Expr::Or(a, b) => eval_operand(a, circuit, cache) | eval_operand(b, circuit, cache),
        Expr::LShift(a, n) => eval_operand(a, circuit, cache) << n,
        Expr::RShift(a, n) => eval_operand(a, circuit, cache) >> n,
        Expr::Not(o) => !eval_operand(o, circuit, cache),
    };
    cache.insert(name.to_string(), v);
    v
}

pub fn process_part1(input: &str) -> String {
    let circuit: HashMap<String, Expr> = parse_circuit(input);
    let mut cache: HashMap<String, u16> = HashMap::new();
    let a: u16 = eval_wire("a", &circuit, &mut cache);
    a.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut circuit: HashMap<String, Expr> = parse_circuit(input);
    let mut cache: HashMap<String, u16> = HashMap::new();
    let signal_on_a: u16 = eval_wire("a", &circuit, &mut cache);
    circuit.insert(
        "b".to_string(),
        Expr::Signal(Operand::Lit(signal_on_a)),
    );
    let mut cache: HashMap<String, u16> = HashMap::new();
    let new_a: u16 = eval_wire("a", &circuit, &mut cache);
    new_a.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn part1_example() {
        let mut circuit: HashMap<String, Expr> = HashMap::new();
        for line in EXAMPLE.lines() {
            let line: &str = line;
            if let Some(parsed) = parse_line(line) {
                let (target, expr): (String, Expr) = parsed;
                circuit.insert(target, expr);
            }
        }
        let mut cache: HashMap<String, u16> = HashMap::new();
        assert_eq!(eval_wire("d", &circuit, &mut cache), 72);
        cache.clear();
        assert_eq!(eval_wire("e", &circuit, &mut cache), 507);
        cache.clear();
        assert_eq!(eval_wire("f", &circuit, &mut cache), 492);
        cache.clear();
        assert_eq!(eval_wire("g", &circuit, &mut cache), 114);
        cache.clear();
        assert_eq!(eval_wire("h", &circuit, &mut cache), 65412);
        cache.clear();
        assert_eq!(eval_wire("i", &circuit, &mut cache), 65079);
    }

    #[test]
    fn part1_works() {
        let result: String = process_part1(include_str!("../input.txt"));
        assert!(!result.is_empty());
        assert!(result.parse::<u32>().is_ok());
    }

    #[test]
    fn part2_override_b_changes_a() {
        let input: &str = "2 -> b
b LSHIFT 1 -> a";
        assert_eq!(process_part1(input), "4");
        assert_eq!(process_part2(input), "8");
    }

    #[test]
    fn part2_works() {
        let input: &str = include_str!("../input.txt");
        let result: String = process_part2(input);
        assert!(!result.is_empty());
        assert!(result.parse::<u32>().is_ok());
    }
}
