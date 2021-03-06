use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop,
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct InstructionParseError;

impl Error for InstructionParseError {}

impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown instruction")
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let op = parts.next().unwrap();
        let arg = parts.next().unwrap();

        match op {
            "acc" => Ok(Instruction::Acc(arg.parse().unwrap())),
            "nop" => Ok(Instruction::Nop),
            "jmp" => Ok(Instruction::Jmp(arg.parse().unwrap())),
            _ => Err(InstructionParseError),
        }
    }
}

struct Execution<'a> {
    accumulator: i32,
    next_instruction_idx: usize,
    programm: &'a Program,
    log: Vec<usize>,
}

#[derive(Debug, PartialEq)]
enum ExecutionResult {
    Running(i32),
    EndlessLoop(i32),
    Terminated(i32),
}

// copied from: https://stackoverflow.com/questions/54035728/how-to-add-a-negative-i32-number-to-an-usize-variable -> thanks @Boiethios
fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

impl<'a> Execution<'_> {
    fn new(p: &'a Program) -> Execution<'a> {
        Execution {
            accumulator: 0,
            next_instruction_idx: 0,
            programm: p,
            log: Vec::new(),
        }
    }

    fn next(&mut self) -> ExecutionResult {
        match self.log.contains(&self.next_instruction_idx) {
            true => ExecutionResult::EndlessLoop(self.accumulator),
            false => {
                self.step();
                if &self.next_instruction_idx >= &self.programm.instructions.len() {
                    ExecutionResult::Terminated(self.accumulator)
                } else {
                    ExecutionResult::Running(self.accumulator)
                }
            }
        }
    }

    fn step(&mut self) {
        assert!(!self.log.contains(&self.next_instruction_idx));
        let inst = &self.programm.instructions[self.next_instruction_idx];
        self.log.push(self.next_instruction_idx);
        match inst {
            Instruction::Nop => self.next_instruction_idx += 1,
            Instruction::Acc(c) => {
                self.accumulator += c;
                self.next_instruction_idx += 1
            }
            Instruction::Jmp(j) => self.next_instruction_idx = add(self.next_instruction_idx, *j),
        }
    }

    fn run(&mut self) -> ExecutionResult {
        loop {
            let r = self.next();
            match r {
                ExecutionResult::EndlessLoop(_) => return r,
                ExecutionResult::Terminated(_) => return r,
                ExecutionResult::Running(_) => (),
            }
        }
    }
}

#[test]
fn testing() {
    println!(">>>>>>>");
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    let program = Program {
        instructions: input
            .lines()
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect(),
    };

    println!("Program: {:#?}", program);

    let mut exe = Execution::new(&program);
    println!("-> {:#?}", exe.run());
}

#[test]
fn test_minimal_term() {
    let input = "nop +0
acc +10";
    let program = Program {
        instructions: input
            .lines()
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect(),
    };

    let mut exe = Execution::new(&program);
    assert_eq!(ExecutionResult::Running(0), exe.next());
    assert_eq!(ExecutionResult::Terminated(10), exe.next());
}

fn part1(program: &Program) {
    println!("Programm with {} instructions.", program.instructions.len());

    let mut exe = Execution::new(&program);
    let mut i = 0;

    while let ExecutionResult::Running(s) = exe.next() {
        println!("i: {} -> {:#?}", i, s);
        i += 1;
    }
}

fn main() {
    // testing();
    let input = include_str!("../input");
    let instructions: Vec<_> = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect();

    let program = Program {
        instructions: instructions.clone(),
    };

    let jumps: Vec<_> = instructions
        .iter()
        .enumerate()
        .filter(|(_idx, i)| match i {
            Instruction::Jmp(_) => true,
            _ => false,
        })
        .map(|(idx, _i)| idx)
        .collect();

    // flip one jump at a time
    println!("looking for valid program by replacing jumps with noops");
    for idx in jumps {
        // println!("Replacing Jmp with Nop in line {}", idx);
        let mut mod_instructions = instructions.clone();
        if let Some(i) = mod_instructions.get_mut(idx) {
            *i = Instruction::Nop;
        }
        let program = Program {
            instructions: mod_instructions,
        };
        let mut exe = Execution::new(&program);
        match exe.run() {
            ExecutionResult::Terminated(acc) => {
                println!("Finished with {}! ", acc);
                break;
            }
            _ => print!("."),
        }
    }

    // part1(&program);
}
