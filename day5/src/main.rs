use std::fmt::Debug;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Mode {
    Position,
    Immediate
}

trait Instruction: Debug {
    fn execute(&self, program: &mut Vec<i32>, ip: usize) -> usize;
}

#[derive(Debug)]
struct Add {
    lhs: i32,
    rhs: i32,
    result: usize,
}
#[derive(Debug)]
struct Mul {
    lhs: i32,
    rhs: i32,
    result: usize,
}
#[derive(Debug)]
struct Input {
    result: usize,
}
#[derive(Debug)]
struct Output {
    what: i32,
}

#[derive(Debug)]
struct JumpIfTrue {
    value: i32,
    result: i32,
}

#[derive(Debug)]
struct JumpIfFalse {
    value: i32,
    result: i32,
}
#[derive(Debug)]
struct LessThan {
    lhs: i32,
    rhs: i32,
    result: usize,
}
#[derive(Debug)]
struct Equals {
    lhs: i32,
    rhs: i32,
    result: usize,
}

impl Instruction for Add {
    fn execute(&self, program: &mut Vec<i32>, ip: usize) ->usize {
        *get_mut_at(program, self.result) = self.lhs +  self.rhs;
        ip + 4
    }
}

impl Instruction for Mul {
    fn execute(&self, program: &mut Vec<i32>, ip: usize) ->usize {
        *get_mut_at(program, self.result) = self.lhs * self.rhs;
        ip + 4
    }
}

impl Instruction for Input {
    fn execute(&self, program: &mut Vec<i32>, ip: usize) ->usize {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        *get_mut_at(program, self.result) = str::parse::<i32>(buffer.trim()).unwrap();
        println!("Input: {}", *get_mut_at(program, self.result));
        ip + 2
    }
}

impl Instruction for Output {
    fn execute(&self, program: &mut Vec<i32>, ip: usize) ->usize {
        println!("Output: {}", self.what);
        ip + 2
    }
}

impl Instruction for JumpIfTrue {
    fn execute(&self, program: &mut Vec<i32>, ip: usize) ->usize {
        if self.value != 0 {
            return self.result as usize;
        }
        ip + 3
    }
}

impl Instruction for JumpIfFalse {
    fn execute(&self, program: &mut Vec<i32>, ip: usize) ->usize {
        if self.value == 0 {
            return self.result as usize;
        }
        ip + 3
    }
}

impl Instruction for LessThan {
    fn execute(&self, program: &mut Vec<i32>, ip: usize) ->usize {
        if self.lhs < self.rhs {
            *get_mut_at(program, self.result) = 1;
        } else {
            *get_mut_at(program, self.result) = 0;
        }
        ip + 4
    }
}

impl Instruction for Equals {
    fn execute(&self, program: &mut Vec<i32>, ip: usize) ->usize {
        if self.lhs == self.rhs {
            *get_mut_at(program, self.result) = 1;
        } else {
            *get_mut_at(program, self.result) = 0;
        }
        ip + 4
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
        (3, _, _, _) => Some(Box::new(Input{result: ip + 1})),
        (4, m1, _, _) => Some(Box::new(Output{what: get_value(program, ip + 1, m1)})),
        (5, m1, m2, _) => Some(Box::new(JumpIfTrue{value: get_value(program, ip + 1, m1), result: get_value(program, ip + 2, m2)})),
        (6, m1, m2, _) => Some(Box::new(JumpIfFalse{value: get_value(program, ip + 1, m1), result: get_value(program, ip + 2, m2)})),
        (7, m1, m2, _) => Some(Box::new(LessThan{lhs: get_value(program, ip + 1, m1), rhs: get_value(program, ip + 2, m2), result: ip + 3})),
        (8, m1, m2, _) => Some(Box::new(Equals{lhs: get_value(program, ip + 1, m1), rhs: get_value(program, ip + 2, m2), result: ip + 3})),
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

fn process() -> i32 {
    let mut input: Vec<i32> = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1101,81,30,225,1102,9,63,225,1001,92,45,224,101,-83,224,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,1102,41,38,225,1002,165,73,224,101,-2920,224,224,4,224,102,8,223,223,101,4,224,224,1,223,224,223,1101,18,14,224,1001,224,-32,224,4,224,1002,223,8,223,101,3,224,224,1,224,223,223,1101,67,38,225,1102,54,62,224,1001,224,-3348,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,1,161,169,224,101,-62,224,224,4,224,1002,223,8,223,101,1,224,224,1,223,224,223,2,14,18,224,1001,224,-1890,224,4,224,1002,223,8,223,101,3,224,224,1,223,224,223,1101,20,25,225,1102,40,11,225,1102,42,58,225,101,76,217,224,101,-153,224,224,4,224,102,8,223,223,1001,224,5,224,1,224,223,223,102,11,43,224,1001,224,-451,224,4,224,1002,223,8,223,101,6,224,224,1,223,224,223,1102,77,23,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,8,226,677,224,1002,223,2,223,1006,224,329,1001,223,1,223,7,226,226,224,102,2,223,223,1006,224,344,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,359,101,1,223,223,1107,226,677,224,1002,223,2,223,1005,224,374,101,1,223,223,1008,677,226,224,1002,223,2,223,1005,224,389,101,1,223,223,1007,677,226,224,1002,223,2,223,1005,224,404,1001,223,1,223,1107,677,226,224,1002,223,2,223,1005,224,419,1001,223,1,223,108,677,226,224,102,2,223,223,1006,224,434,1001,223,1,223,7,226,677,224,102,2,223,223,1005,224,449,1001,223,1,223,107,226,226,224,102,2,223,223,1006,224,464,101,1,223,223,107,677,226,224,102,2,223,223,1006,224,479,101,1,223,223,1007,677,677,224,1002,223,2,223,1006,224,494,1001,223,1,223,1008,226,226,224,1002,223,2,223,1006,224,509,101,1,223,223,7,677,226,224,1002,223,2,223,1006,224,524,1001,223,1,223,1007,226,226,224,102,2,223,223,1006,224,539,101,1,223,223,8,677,226,224,1002,223,2,223,1006,224,554,101,1,223,223,1008,677,677,224,102,2,223,223,1006,224,569,101,1,223,223,1108,677,226,224,102,2,223,223,1005,224,584,101,1,223,223,107,677,677,224,102,2,223,223,1006,224,599,1001,223,1,223,1108,677,677,224,1002,223,2,223,1006,224,614,1001,223,1,223,1107,677,677,224,1002,223,2,223,1005,224,629,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,644,101,1,223,223,8,226,226,224,1002,223,2,223,1005,224,659,101,1,223,223,1108,226,677,224,1002,223,2,223,1006,224,674,101,1,223,223,4,223,99,226];

    let mut ip = 0;
    while let Some(instruction) = parse_instruction(&mut input, ip) {
 //       println!("DBG: {} {:?}" ,ip,  instruction);
        ip = instruction.execute(&mut input, ip);
 //       println!("State: {:?}" ,input);
    }

    input[0]
}

fn main() {
    process();
}

#[cfg(test)]
mod test {
    use super::parse_opcode;
    use crate::Mode::{Position, Immediate};

    #[test]
    fn  parsing_opcodes() {
        assert_eq!((2, Position, Immediate, Position), parse_opcode(1002));
    }
}