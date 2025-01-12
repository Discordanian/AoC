#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Parameter {
    Immediate(i32),
    Positional(usize),
}

#[derive(Debug, Clone)]
pub struct Computer {
    pub memory: Vec<i32>,
    pub ibuff: Vec<i32>,
    pub obuff: Vec<i32>,
    pub program_pointer: usize,
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Add(Parameter, Parameter, Parameter),
    Mult(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpNonZero(Parameter, Parameter),
    JumpZero(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Halt,
}

impl Parameter {
    fn new(opcode: i32, position: i32, value: i32) -> Self {
        let mode = (opcode / 10_i32.pow(position as u32 + 1)) % 10;
        match mode {
            0 => Self::Positional(value as usize),
            1 => Self::Immediate(value),
            _ => panic!("Unknown parameter mode received"),
        }
    }
}

impl std::ops::Index<usize> for Computer {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

impl std::ops::IndexMut<usize> for Computer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory[index]
    }
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
    fn parameter_pointer(&self, parameter: Parameter) -> usize {
        match parameter {
            Parameter::Positional(ptr) => ptr,
            Parameter::Immediate(_) => panic!("No such thing as immediate pointer"),
        }
    }
    pub fn new_with_input(memory: &str, input: &[i32]) -> Self {
        Computer {
            memory: Computer::memory_parse(memory),
            ibuff: input.to_vec(),
            obuff: Vec::new(),
            program_pointer: 0,
        }
    }
    pub fn next(&mut self) -> Option<Opcode> {
        if self.program_pointer >= self.memory.len() {
            return None;
        }
        let mem = self.memory[self.program_pointer];
        let op = mem % 100;
        if op == 99 {
            return None;
        }

        let opcode = match op {
            1 => Opcode::Add(
                Parameter::new(mem, 1, self[self.program_pointer + 1]),
                Parameter::new(mem, 2, self[self.program_pointer + 2]),
                Parameter::new(mem, 3, self[self.program_pointer + 3]),
            ),
            2 => Opcode::Mult(
                Parameter::new(mem, 1, self[self.program_pointer + 1]),
                Parameter::new(mem, 2, self[self.program_pointer + 2]),
                Parameter::new(mem, 3, self[self.program_pointer + 3]),
            ),
            3 => Opcode::Input(Parameter::new(mem, 1, self[self.program_pointer + 1])),
            4 => Opcode::Output(Parameter::new(mem, 1, self[self.program_pointer + 1])),
            5 => Opcode::JumpNonZero(
                Parameter::new(mem, 1, self[self.program_pointer + 1]),
                Parameter::new(mem, 2, self[self.program_pointer + 2]),
            ),
            6 => Opcode::JumpZero(
                Parameter::new(mem, 1, self[self.program_pointer + 1]),
                Parameter::new(mem, 2, self[self.program_pointer + 2]),
            ),
            7 => Opcode::LessThan(
                Parameter::new(mem, 1, self[self.program_pointer + 1]),
                Parameter::new(mem, 2, self[self.program_pointer + 2]),
                Parameter::new(mem, 3, self[self.program_pointer + 3]),
            ),
            8 => Opcode::Equals(
                Parameter::new(mem, 1, self[self.program_pointer + 1]),
                Parameter::new(mem, 2, self[self.program_pointer + 2]),
                Parameter::new(mem, 3, self[self.program_pointer + 3]),
            ),
            99 => Opcode::Halt,
            x => panic!("Unknown Opcode {x}"),
        };
        self.program_pointer += match op {
            99 => 1,
            1 | 2 | 7 | 8 => 4,
            3 | 4 => 2,
            5 | 6 => 3,
            _ => panic!("Not sure how to update program pointer based on opcode {op}"),
        };
        Some(opcode)
    }
    pub fn execute(&mut self) -> &Self {
        assert!(!self.memory.is_empty());

        while let Some(opcode) = self.next() {
            /*
            dbg!((
                &opcode,
                &self.program_pointer,
                &self.memory[0..self.program_pointer]
            ));
            */
            match opcode {
                Opcode::Halt => {
                    unreachable!("Should never get a Halt because a Halt should return None")
                }
                Opcode::Add(p1, p2, p3) => {
                    let p1 = match p1 {
                        Parameter::Immediate(v) => v,
                        Parameter::Positional(ptr) => self[ptr],
                    };
                    let p2 = match p2 {
                        Parameter::Immediate(v) => v,
                        Parameter::Positional(ptr) => self[ptr],
                    };
                    let ptr = self.parameter_pointer(p3);
                    self[ptr] = p1 + p2;
                }
                Opcode::Mult(p1, p2, p3) => {
                    let p1 = match p1 {
                        Parameter::Immediate(v) => v,
                        Parameter::Positional(ptr) => self[ptr],
                    };
                    let p2 = match p2 {
                        Parameter::Immediate(v) => v,
                        Parameter::Positional(ptr) => self[ptr],
                    };
                    let ptr = self.parameter_pointer(p3);
                    self[ptr] = p1 * p2;
                }
                Opcode::Input(p1) => {
                    let x = self
                        .ibuff
                        .pop()
                        .expect("Input has a value to read from when requested");
                    let ptr = self.parameter_pointer(p1);
                    self[ptr] = x;
                }
                Opcode::Output(p1) => {
                    let p1: i32 = match p1 {
                        Parameter::Immediate(v) => v,
                        Parameter::Positional(ptr) => self[ptr],
                    };
                    self.obuff.push(p1);
                }
                Opcode::JumpNonZero(p1, p2) => {
                    let p1 = match p1 {
                        Parameter::Immediate(v) => v,
                        Parameter::Positional(ptr) => self[ptr],
                    };
                    let ptr: usize = match p2 {
                        Parameter::Immediate(v) => v as usize,
                        Parameter::Positional(ptr) => self[ptr] as usize,
                    };
                    // let ptr = self.parameter_pointer(p2);
                    if p1 != 0 {
                        self.program_pointer = ptr;
                    }
                }
                Opcode::JumpZero(p1, p2) => {
                    let p1 = match p1 {
                        Parameter::Immediate(v) => v,
                        Parameter::Positional(ptr) => self[ptr],
                    };
                    let ptr: usize = match p2 {
                        Parameter::Immediate(v) => v as usize,
                        Parameter::Positional(ptr) => self[ptr] as usize,
                    };
                    if p1 == 0 {
                        self.program_pointer = ptr;
                    }
                }
                Opcode::LessThan(p1, p2, p3) => {
                    let p1 = match p1 {
                        Parameter::Immediate(v) => v,
                        Parameter::Positional(ptr) => self[ptr],
                    };
                    let p2 = match p2 {
                        Parameter::Immediate(v) => v,
                        Parameter::Positional(ptr) => self[ptr],
                    };
                    let ptr = self.parameter_pointer(p3);
                    if p1 < p2 {
                        self[ptr] = 1;
                    } else {
                        self[ptr] = 0;
                    }
                }
                Opcode::Equals(p1, p2, p3) => {
                    let p1 = match p1 {
                        Parameter::Immediate(v) => v,
                        Parameter::Positional(ptr) => self[ptr],
                    };
                    let p2 = match p2 {
                        Parameter::Immediate(v) => v,
                        Parameter::Positional(ptr) => self[ptr],
                    };
                    let ptr = self.parameter_pointer(p3);
                    if p1 == p2 {
                        self[ptr] = 1;
                    } else {
                        self[ptr] = 0;
                    }
                }
            }
        }

        self
    }
}
