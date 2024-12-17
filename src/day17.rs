use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::example_tests;

#[derive(Debug, Clone)]
struct Machine {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    ip: usize,
    program: Vec<u8>,
}

// Opcodes
//
// The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
// The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
// The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

impl Machine {
    fn fetch(&mut self) -> Option<(u8, u8)> {
        let opcode = *self.program.get(self.ip)?;
        let operand = *self.program.get(self.ip + 1)?;
        debug_assert!(opcode < 8);
        self.ip += 2;
        Some((opcode, operand))
    }

    fn combo_operand(&self, operand: u8) -> u64 {
        // Combo operands 0 through 3 represent literal values 0 through 3.
        // Combo operand 4 represents the value of register A.
        // Combo operand 5 represents the value of register B.
        // Combo operand 6 represents the value of register C.
        // Combo operand 7 is reserved and will not appear in valid programs.
        match operand {
            0..=3 => operand as _,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => unreachable!("Invalid combo operand: {}", operand),
        }
    }

    fn execute(&mut self, opcode: u8, operand: u8) -> Option<u8> {
        match opcode {
            ADV => {
                let combo = self.combo_operand(operand);
                let divisor = 2u64.pow(combo as _);
                self.register_a /= divisor;
            }
            BXL => {
                self.register_b ^= operand as u64;
            }
            BST => {
                self.register_b = self.combo_operand(operand) as u64 % 8;
            }
            JNZ => {
                if self.register_a != 0 {
                    self.ip = operand as _;
                }
            }
            BXC => {
                self.register_b ^= self.register_c;
            }
            OUT => {
                let value = self.combo_operand(operand) as u8 % 8;
                return Some(value);
            }
            BDV => {
                let combo = self.combo_operand(operand);
                let divisor = 2u64.pow(combo as _);
                self.register_b = self.register_a / divisor;
            }
            CDV => {
                let combo = self.combo_operand(operand);
                let divisor = 2u64.pow(combo as _);
                self.register_c = self.register_a / divisor;
            }
            _ => unreachable!("Invalid opcode: {}", opcode),
        }
        None
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Machine {
    let mut lines = input.lines();
    let register_a = lines
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    let register_b = lines
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    let register_c = lines
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();
    let _ = lines.next().unwrap();
    let program = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    Machine {
        register_a,
        register_b,
        register_c,
        ip: 0,
        program,
    }
}

#[derive(Debug)]
struct ProgramOutput(Vec<u8>);

impl ProgramOutput {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, value: u8) {
        self.0.push(value);
    }
}

impl std::fmt::Display for ProgramOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for x in &self.0 {
            if first {
                first = false;
                write!(f, "{x}")?;
            } else {
                write!(f, ",{x}")?;
            }
        }
        Ok(())
    }
}

#[aoc(day17, part1)]
fn part1(input: &Machine) -> String {
    let mut out = ProgramOutput::new();
    let mut machine = input.clone();
    while let Some((opcode, operand)) = machine.fetch() {
        if let Some(value) = machine.execute(opcode, operand) {
            out.push(value);
        }
    }
    out.to_string()
}

#[aoc(day17, part2)]
fn part2(input: &Machine) -> String {
    todo!()
}

example_tests! {
    "
    Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0
    ",

    part1 => "4,6,3,5,6,3,5,2,1,0",
}
