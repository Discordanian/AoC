#[derive(Clone, Copy)]
enum Reg {
    A,
    B,
}

enum Instr {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(i32),
    Jie(Reg, i32),
    Jio(Reg, i32),
}

fn parse_reg(s: &str) -> Reg {
    match s {
        "a" => Reg::A,
        "b" => Reg::B,
        _ => panic!("unknown register: {s}"),
    }
}

fn parse_line(line: &str) -> Option<Instr> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    let parts: Vec<&str> = line.split_whitespace().collect();
    Some(match parts[0] {
        "hlf" => Instr::Hlf(parse_reg(parts[1])),
        "tpl" => Instr::Tpl(parse_reg(parts[1])),
        "inc" => Instr::Inc(parse_reg(parts[1])),
        "jmp" => Instr::Jmp(parts[1].parse().expect("jmp offset")),
        "jie" => {
            let r = parts[1].trim_end_matches(',');
            Instr::Jie(
                parse_reg(r),
                parts[2].parse().expect("jie offset"),
            )
        }
        "jio" => {
            let r = parts[1].trim_end_matches(',');
            Instr::Jio(
                parse_reg(r),
                parts[2].parse().expect("jio offset"),
            )
        }
        other => panic!("unknown instruction: {other}"),
    })
}

fn run(program: &[Instr], mut a: u64, mut b: u64) -> (u64, u64) {
    let mut pc: i32 = 0;
    let len = program.len() as i32;

    while pc >= 0 && pc < len {
        let i = pc as usize;
        let mut next = pc + 1;
        match &program[i] {
            Instr::Hlf(r) => match r {
                Reg::A => a /= 2,
                Reg::B => b /= 2,
            },
            Instr::Tpl(r) => match r {
                Reg::A => a *= 3,
                Reg::B => b *= 3,
            },
            Instr::Inc(r) => match r {
                Reg::A => a += 1,
                Reg::B => b += 1,
            },
            Instr::Jmp(offset) => next = pc + offset,
            Instr::Jie(r, offset) => {
                let v = match r {
                    Reg::A => a,
                    Reg::B => b,
                };
                if v % 2 == 0 {
                    next = pc + offset;
                }
            }
            Instr::Jio(r, offset) => {
                let v = match r {
                    Reg::A => a,
                    Reg::B => b,
                };
                if v == 1 {
                    next = pc + offset;
                }
            }
        }
        pc = next;
    }

    (a, b)
}

pub fn process_part1(input: &str) -> String {
    let program: Vec<Instr> = input.lines().filter_map(parse_line).collect();
    let (_a, b) = run(&program, 0, 0);
    b.to_string()
}

pub fn process_part2(input: &str) -> String {
    input.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_from_description() {
        let input = "inc a\njio a, +2\ntpl a\ninc a\n";
        let program: Vec<Instr> = input.lines().filter_map(parse_line).collect();
        let (a, b) = run(&program, 0, 0);
        assert_eq!(a, 2);
        assert_eq!(b, 0);
    }
}
