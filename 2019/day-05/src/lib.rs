#[derive(Debug, Clone)]
pub struct Computer {
    memory: Vec<i32>,
    ibuff: Vec<i32>,
    obuff: Vec<i32>,
    program_pointer: usize,
}

pub enum Opcode {
    Add(i32, i32, usize),
    Mult(i32, i32, usize),
    Input(usize),
    Output(i32),
    JumpNonZero(i32, usize),
    JumpZero(i32, usize),
    LessThan(i32, i32, usize),
    Equals(i32, i32, usize),
    Halt,
}

impl Computer {
    fn memory_parse(input: &str) -> Vec<i32> {
        input
            .trim()
            .split(",")
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse().expect("Able to convert memory to i32"))
            .collect()
    }
    pub fn init_from_str(memory: &str) -> Computer {
        Computer {
            memory: Computer::memory_parse(memory),
            ibuff: Vec::new(),
            obuff: Vec::new(),
            program_pointer: 0,
        }
    }
    pub fn get_value(&self, position_mode: bool, ptr: usize) -> i32 {
        match position_mode {
            true => self.memory[self.memory[ptr] as usize],
            false => self.memory[ptr],
        }
    }
    pub fn get_opcode(&mut self) -> Opcode {
        let mem = self.memory[self.program_pointer];
        let op = mem % 100;

        // position_modes is a tuple
        // position mode param 1, position mode param 2, position mode param 3
        let position_modes: (bool, bool, bool) = match mem / 100 {
            111 => (false, false, false),
            110 => (true, false, false),
            101 => (true, false, true),
            100 => (true, true, false),
            11 => (false, false, true),
            10 => (true, false, true),
            1 => (false, true, true),
            0 => (true, true, true),
            x => panic!("Unknown mode code {x}"),
        };

        match op {
            1 => Opcode::Add(
                self.get_value(position_modes.0, self.program_pointer + 1),
                self.get_value(position_modes.1, self.program_pointer + 2),
                self.get_value(position_modes.2, self.program_pointer + 3) as usize,
            ),
            2 => Opcode::Mult(
                self.get_value(position_modes.0, self.program_pointer + 1),
                self.get_value(position_modes.1, self.program_pointer + 2),
                self.get_value(position_modes.2, self.program_pointer + 3) as usize,
            ),
            3 => Opcode::Input(self.get_value(position_modes.0, self.program_pointer + 1) as usize),
            4 => Opcode::Output(self.get_value(position_modes.0, self.program_pointer + 1)),
            5 => Opcode::JumpNonZero(
                self.get_value(position_modes.0, self.program_pointer + 1),
                self.get_value(position_modes.1, self.program_pointer + 2) as usize,
            ),
            6 => Opcode::JumpZero(
                self.get_value(position_modes.0, self.program_pointer + 1),
                self.get_value(position_modes.1, self.program_pointer + 2) as usize,
            ),
            7 => Opcode::LessThan(
                self.get_value(position_modes.0, self.program_pointer + 1),
                self.get_value(position_modes.1, self.program_pointer + 2),
                self.get_value(position_modes.2, self.program_pointer + 3) as usize,
            ),
            8 => Opcode::Equals(
                self.get_value(position_modes.0, self.program_pointer + 1),
                self.get_value(position_modes.1, self.program_pointer + 2),
                self.get_value(position_modes.2, self.program_pointer + 3) as usize,
            ),
            99 => Opcode::Halt,
            x => panic!("Unknown Opcode {x}"),
        }
    }
    pub fn execute(&mut self) -> &Self {
        assert!(!self.memory.is_empty());

        loop {
            let opcode = self.get_opcode();
            match opcode {
                Opcode::Halt => {
                    self.program_pointer += 1;
                    break;
                }
                Opcode::Add(p1, p2, ptr) => {
                    self.program_pointer += 4;
                    self.memory[ptr] = p1 + p2;
                }
                Opcode::Mult(p1, p2, ptr) => {
                    self.program_pointer += 4;
                    self.memory[ptr] = p1 * p2;
                }
                Opcode::Input(ptr) => {
                    self.program_pointer += 2;
                    self.memory[ptr] = self.ibuff.pop().expect("Input expects IO to have value");
                }
                Opcode::Output(value) => {
                    self.program_pointer += 2;
                    self.obuff.push(value);
                }
                Opcode::JumpNonZero(value, ptr) => match value {
                    0 => self.program_pointer += 2,
                    _ => self.program_pointer = ptr,
                },
                Opcode::JumpZero(value, ptr) => match value {
                    0 => self.program_pointer = ptr,
                    _ => self.program_pointer += 2,
                },
                Opcode::LessThan(p1, p2, ptr) => match p1 < p2 {
                    true => self.memory[ptr] = 1,
                    false => self.memory[ptr] = 0,
                },
                Opcode::Equals(p1, p2, ptr) => match p1 == p2 {
                    true => self.memory[ptr] = 1,
                    false => self.memory[ptr] = 0,
                },
            }
        }

        self
    }
}

pub fn process_part1(input: &str) -> i32 {
    let mut computer: Computer = Computer::init_from_str(input);
    computer.ibuff.push(1);
    computer.execute();

    computer.obuff.pop().expect("Output Buffer has value")
}

pub fn process_part2(input: &str) -> i32 {
    let mut computer: Computer = Computer::init_from_str(input);
    computer.ibuff.push(5);
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
