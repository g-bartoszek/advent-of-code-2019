
#[derive(Copy, Clone, PartialEq, Debug)]
enum Mode {
    Position,
    Immediate
}

trait Instruction {
    fn size(&self) -> usize;
    fn execute(&self, program: &mut Vec<i32>);

}

struct Add {
    lhs: i32,
    rhs: i32,
    result: usize,
}
struct Mul {
    lhs: i32,
    rhs: i32,
    result: usize,
}

impl Instruction for Add {
    fn size(&self) -> usize {
        4
    }
    fn execute(&self, program: &mut Vec<i32>) {
        *get_mut_at(program, self.result) = self.lhs +  self.rhs;
    }
}

impl Instruction for Mul {
    fn size(&self) -> usize {
        4
    }

    fn execute(&self, program: &mut Vec<i32>) {
        *get_mut_at(program, self.result) = self.lhs * self.rhs;
    }
}

fn get_mut_at(program: &mut Vec<i32>, index: usize) -> &mut i32 {
    let lvl2_index =  program[index] as usize;
    &mut program[lvl2_index]
}

fn get_value(program: &mut Vec<i32>, index: usize, mode: Mode) -> i32 {
    match mode {
        Mode::Immediate => program[index],
        Mode::Position => *get_mut_at(program, index),
    }
}

fn parse_instruction(program: &mut Vec<i32>, ip: usize) -> Option<Box<dyn Instruction>> {
    match parse_opcode(program[ip]) {
        (1, m1, m2, _) => Some(Box::new(Add{lhs: get_value(program, ip + 1, m1), rhs: get_value(program, ip + 2, m2), result: ip + 3})),
        (2, m1, m2, _) => Some(Box::new(Mul{lhs: get_value(program, ip + 1, m1), rhs: get_value(program, ip + 2, m2), result: ip + 3})),
        (99, _, _,_) => None,
        _ => panic!("Unrecognized instruction!")
    }
}

fn parse_opcode(mut opcode: i32) -> (i32, Mode, Mode, Mode) {
    let mode3 = if opcode > 10000 {Mode::Immediate} else {Mode::Position};
    opcode = opcode % 10000;
    let mode2 = if opcode > 1000 {Mode::Immediate} else {Mode::Position};
    opcode = opcode % 1000;
    let mode1 = if opcode > 100 {Mode::Immediate} else {Mode::Position};
    opcode = opcode % 100;

    (opcode, mode1, mode2, mode3)
}

fn process(noun: i32, verb: i32) -> i32 {
    let mut input: Vec<i32> = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,6,19,23,2,23,6,27,2,6,27,31,2,13,31,35,1,10,35,39,2,39,13,43,1,43,13,47,1,6,47,51,1,10,51,55,2,55,6,59,1,5,59,63,2,9,63,67,1,6,67,71,2,9,71,75,1,6,75,79,2,79,13,83,1,83,10,87,1,13,87,91,1,91,10,95,2,9,95,99,1,5,99,103,2,10,103,107,1,107,2,111,1,111,5,0,99,2,14,0,0];
    input[1] = noun;
    input[2] = verb;

    let mut ip = 0;
    while let Some(instruction) = parse_instruction(&mut input, ip) {
        instruction.execute(&mut input);
        ip += instruction.size();
    }

    input[0]
}

fn main() {
    for n in 0..99 {
        for v in 0..99 {
            if process(n, v) == 19690720 {
                assert_eq!(4559, 100 * n + v);
                println!("Got it: {}", 100 * n + v);
                return;
            }
        }
    }

}

#[cfg(test)]
mod Test {
    use super::parse_opcode;
    use crate::Mode::{Position, Immediate};

    #[test]
    fn  parsing_opcodes() {
        assert_eq!((2, Position, Immediate, Position), parse_opcode(1002));
    }
}