use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{example_tests, known_input_tests};

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

    // fn as_number(&self) -> u64 {
    //     self.0.iter().rev().fold(0, |acc, &x| acc * 8 + x as u64)
    // }
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

impl PartialEq<&str> for ProgramOutput {
    fn eq(&self, other: &&str) -> bool {
        let s = self.to_string();
        s == *other
    }
}

#[aoc(day17, part1)]
fn part1(input: &Machine) -> ProgramOutput {
    let mut out = ProgramOutput::new();
    let mut machine = input.clone();
    while let Some((opcode, operand)) = machine.fetch() {
        if let Some(value) = machine.execute(opcode, operand) {
            out.push(value);
        }
    }
    out
}

#[aoc(day17, part2)]
fn part2(input: &Machine) -> usize {
    if input.program.len() > 6 {
        return 0;
    }
    for i in (0..=u32::MAX).rev() {
        let mut out = ProgramOutput::new();
        let mut machine = input.clone();
        machine.register_a = i as _;
        while let Some((opcode, operand)) = machine.fetch() {
            if let Some(value) = machine.execute(opcode, operand) {
                out.push(value);
            }
        }
        println!("{i}: {out}");
        if out.0 == input.program {
            return i as _;
        }
    }
    panic!()
}

fn hardcoded_part2_machine(n: u64) -> u8 {
    // bst 4 | B = A % 8
    // bxl 3 | B = B ^ 3
    // cdv 5 | C = A / 2^B
    // adv 3 | A = A / 8
    // bxl 4 | B = B ^ 4
    // bxc 7 | B = B ^ C
    // out 5 | output B
    // jnz 0 | if a != 0 jump to 0

    // let mut b;
    // let c;
    // b = n & 7;
    // b ^= 3;
    // c = n >> b;
    // b ^= 4;
    // b ^= c;
    // (b & 7) as _
    //
    let x0 = n & 7;
    // let x1 = (n >> 3) & 7;
    // let x2 = (n >> 6) & 7;
    // ((x0 ^ 7) ^ ((x2 * 64 + x1 * 8 + x0) >> (x0 ^ 3)) & 7) as _
    ((x0 ^ 7) ^ (n >> (x0 ^ 3)) & 7) as _
}

#[aoc(day17, part2, p2harcoded)]
fn part2_hardcoded(input: &Machine) -> usize {
    let table = {
        let mut table = [0; 1024];
        for i in 0..1024 {
            table[i] = hardcoded_part2_machine(i as _);
            let foo = part1(&Machine {
                register_a: i as _,
                register_b: 0,
                register_c: 0,
                ip: 0,
                program: input.program.clone(),
            })
            .0[0];
            debug_assert_eq!(foo, table[i] as _);
        }
        table
    };

    let reverse = {
        let mut reverse: [Vec<u16>; 8] = [(); 8].map(|()| Vec::with_capacity(96));
        for i in 0..1024 {
            reverse[table[i] as usize].push(i as _);
        }
        for v in &mut reverse {
            v.sort_unstable();
        }
        reverse
    };

    // debug_assert_eq!(table[0o321], 2);
    // debug_assert_eq!(table[0o654], 0);
    // debug_assert_eq!(table[0o050], 2);
    // debug_assert_eq!(table[0o305], 1);
    debug_assert!(reverse[2].contains(&0o050));
    debug_assert!(reverse[1].contains(&0o305));
    debug_assert!(reverse[4].contains(&0o730));
    debug_assert!(reverse[7].contains(&0o473));
    debug_assert!(reverse[6].contains(&0o747));
    debug_assert!(reverse[0].contains(&0o774));
    debug_assert!(reverse[3].contains(&0o277));
    debug_assert!(reverse[1].contains(&0o027));
    debug_assert!(reverse[4].contains(&0o002));

    debug_assert!(reverse[4].contains(&0o130));
    debug_assert!(reverse[5].contains(&0o1454));

    let expected = input.program.iter().copied().collect::<Vec<_>>();
    // let expected = [2, 1, 4, 7, 6, 0, 3, 1, 4];
    // expected.reverse();

    let mut stack = vec![0];
    let mut result = 0u64;

    let mut solutions = vec![];
    loop {
        let idx = *stack.last().unwrap();
        let depth = stack.len() - 1;

        // println!("{stack:?} {result:o} [expected = {}]", expected[depth]);
        if depth == 0 {
            if let Some((next_match, &digits)) = reverse[expected[depth] as usize]
                .iter()
                .enumerate()
                .skip(idx)
                // .filter(|&(_, &digits)| digits >= 64)
                // .inspect(|(i, digits)| println!("{i:2} {digits:03o}"))
                .next()
            {
                // println!("{digits:0o}");
                *stack.last_mut().unwrap() = next_match as _;
                result = digits as u64;
                stack.push(0);
            } else {
                break;
                // panic!("No solution found");
            }
        } else {
            if let Some((next_match, &digits)) = reverse[expected[depth] as usize]
                .iter()
                .enumerate()
                .skip(idx)
                // .inspect(|(i, digits)| println!("{i:2} {digits:03o}"))
                .find(|(_, digits)| **digits & 0o77 == (result >> (depth * 3)) as u16 & 0o77)
            // .find(|(_, digits)| **digits >> 3 == result as u16 & 0o77)
            {
                // println!("{digits:03o}");
                *stack.last_mut().unwrap() = next_match as _;
                result += (digits as u64 & 0o700) << (depth * 3);
                // result = result * 8 + (digits as u64 & 7);
                if depth == expected.len() - 1 {
                    println!("===> {result:o}");
                    solutions.push(result);
                    stack.pop();
                    *stack.last_mut().unwrap() += 1;
                    // result >>= 3;
                    result &= (1 << ((depth + 1) * 3)) - 1;
                } else {
                    stack.push(0);
                }
            } else {
                // result >>= 3;
                // let before = result;
                result &= (1 << ((depth + 1) * 3)) - 1;
                // println!("< pop! {before:o} => {result:o}\n");
                stack.pop();
                if stack.is_empty() {
                    // panic!("No solution found");
                    break;
                }
                *stack.last_mut().unwrap() += 1;
            }
        }
    }

    solutions.sort();
    for &solution in &solutions {
        println!("Solution: 0o{solution:o} {solution}");

        {
            // check result
            let machine = Machine {
                register_a: solution,
                register_b: 0,
                register_c: 0,
                ip: 0,
                program: input.program.clone(),
            };
            let out = part1(&machine);
            let expected = ProgramOutput(expected.iter().copied().collect());
            let mark = if out.0 == expected.0 { "✅" } else { "❌" };
            println!("     out = {out}");
            println!("expected = {expected} {mark}");
        }
    }

    solutions.iter().min().copied().unwrap() as _
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

mod p2example {
    use super::*;
    example_tests! {
        "
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
        ",

        part2 => 117440,
    }
}

known_input_tests! {
    input: include_str!("../input/2024/day17.txt"),
    part1 => "2,1,4,7,6,0,3,1,4",
    part2_hardcoded => 266932601404433,
}
