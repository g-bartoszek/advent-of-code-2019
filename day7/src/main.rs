use std::fmt::Debug;
use std::iter::FromIterator;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Mode {
    Position(usize),
    Immediate(usize)
}

trait Instruction: Debug {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize);
}

#[derive(Debug)]
struct Add {
    lhs: Mode,
    rhs: Mode,
    result: Mode,
}
#[derive(Debug)]
struct Mul {
    lhs: Mode,
    rhs: Mode,
    result: Mode,
}
#[derive(Debug)]
struct Input {
    input: i32,
    result: Mode,
}
#[derive(Debug)]
struct Output<'a> {
    output: &'a mut Vec<i32>,
    what: Mode,
}

#[derive(Debug)]
struct JumpIfTrue {
    value: Mode,
    result: Mode,
}

#[derive(Debug)]
struct JumpIfFalse {
    value: Mode,
    result: Mode,
}
#[derive(Debug)]
struct LessThan {
    lhs: Mode,
    rhs: Mode,
    result: Mode,
}
#[derive(Debug)]
struct Equals {
    lhs: Mode,
    rhs: Mode,
    result: Mode,
}

impl Instruction for Add {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        *get_value(program, self.result) = *get_value(program,self.lhs) +  *get_value(program, self.rhs);
        *ip += 4
    }
}

impl Instruction for Mul {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        *get_value(program, self.result) = *get_value(program,self.lhs) *  *get_value(program, self.rhs);
        *ip += 4
    }
}

impl Instruction for Input {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        *get_value(program, self.result) = self.input;
        println!("Input: {}", self.input);
        *ip += 2
    }
}

impl Instruction for Output<'_> {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        let o = *get_value(program, self.what);
        self.output.push(o);
        println!("Output: {}", o);
        *ip += 2
    }
}

impl Instruction for JumpIfTrue {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        if *get_value(program, self.value) != 0 {
            *ip = *get_value(program, self.result) as usize;
        }
        *ip += 3
    }
}

impl Instruction for JumpIfFalse {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        if *get_value(program, self.value) == 0 {
            *ip = *get_value(program, self.result) as usize;
        }
        *ip += 3
    }
}

impl Instruction for LessThan {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        if *get_value(program, self.lhs) < *get_value(program, self.rhs) {
            *get_value(program, self.result) = 1;
        } else {
            *get_value(program, self.result) = 0;
        }
        *ip += 4
    }
}

impl Instruction for Equals {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        if *get_value(program, self.lhs) == *get_value(program, self.rhs) {
            *get_value(program, self.result) = 1;
        } else {
            *get_value(program, self.result) = 0;
        }
        *ip += 4
    }
}

fn get_mut_at(program: &mut Vec<i32>, index: usize) -> &mut i32 {
    let lvl2_index =  program[index] as usize;
    &mut program[lvl2_index]
}

fn get_value(program: &mut Vec<i32>, mode: Mode) -> &mut i32 {
    match mode {
        Mode::Immediate(index) => &mut program[index],
        Mode::Position(index) => get_mut_at(program, index),
    }
}

struct Parser{
    input: Vec<i32>,
    output: Vec<i32>,
    program: Vec<i32>,
    ip: usize
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Status {
    Run,
    Halt,
    Output(i32)
}

impl<'a> Parser {

    fn new(program: Vec<i32>, input: Vec<i32>) -> Self {
        Self{input, output: vec![], program, ip: 0usize}
    }

    fn parse_instruction(&'a mut self) -> Status {
        let ip = self.ip;
        let mut opcode = self.program[ip];
        let _mode3 = if opcode > 10000 {Mode::Immediate(ip+3)} else {Mode::Position(ip+3)};
        opcode = opcode % 10000;
        let mode2 = if opcode > 1000 {Mode::Immediate(ip+2)} else {Mode::Position(ip+2)};
        opcode = opcode % 1000;
        let mode1 = if opcode > 100 {Mode::Immediate(ip+1)} else {Mode::Position(ip+1)};
        opcode = opcode % 100;

        let mut status = Status::Run;
        {
            let mut instr: Option<Box<Instruction>> = match opcode {
                1 => Some(Box::new(Add { lhs: mode1, rhs: mode2, result: Mode::Position(ip + 3) })),
                2 => Some(Box::new(Mul { lhs: mode1, rhs: mode2, result: Mode::Position(ip + 3) })),
                3 => Some(Box::new(Input { input: self.input.pop().unwrap(), result: Mode::Position(ip + 1) })),
                4 => Some(Box::new(Output { what: mode1, output: &mut self.output })),
                5 => Some(Box::new(JumpIfTrue { value: mode1, result: mode2 })),
                6 => Some(Box::new(JumpIfFalse { value: mode1, result: mode2 })),
                7 => Some(Box::new(LessThan { lhs: mode1, rhs: mode2, result: Mode::Position(ip + 3) })),
                8 => Some(Box::new(Equals { lhs: mode1, rhs: mode2, result: Mode::Position(ip + 3) })),
                99 => None,
                _ => panic!("Unrecognized instruction!")
            };

            if opcode == 99 {
                println!("HALT");
                return Status::Halt;
            }

            if let Some(mut i) = instr {
                println!("IP: {} INSTR: {:?}",self.ip, i);
                i.execute(&mut self.program, &mut self.ip);
                println!("STATE: {:?}", self.program);
            }
        }

        if opcode == 4 {
            return Status::Output(self.output[0]);
        }


        status
    }

    fn process(&mut self) -> Status {
        let mut status = Status::Run;
        while status == Status::Run {
            status = self.parse_instruction();
        }
        status
    }
}



fn check_permutation(p: &Vec<i32>) {
    let program = vec![3,8,1001,8,10,8,105,1,0,0,21,38,55,64,81,106,187,268,349,430,99999,3,9,101,2,9,9,1002,9,2,9,101,5,9,9,4,9,99,3,9,102,2,9,9,101,3,9,9,1002,9,4,9,4,9,99,3,9,102,2,9,9,4,9,99,3,9,1002,9,5,9,1001,9,4,9,102,4,9,9,4,9,99,3,9,102,2,9,9,1001,9,5,9,102,3,9,9,1001,9,4,9,102,5,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,99];
    //let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33, 1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
    println!("Permutation: {:?}", p);
    if let Status::Output(o1) = Parser::new(program.clone(),vec![0, p[0]]).process() {
        if let Status::Output(o2) = Parser::new(program.clone(), vec![o1, p[1]]).process() {
            if let Status::Output(o3) = Parser::new(program.clone(), vec![o2, p[2]]).process() {
                if let Status::Output(o4) = Parser::new(program.clone(), vec![o3, p[3]]).process() {
                    if let Status::Output(o5) = Parser::new(program.clone(), vec![o4, p[4]]).process() {
                        println!("Result: {}", o5);
                    }
                }
            }
        }
    }
}

fn find_permutation(remaining: std::collections::HashSet<i32>, current: Vec<i32>) {
    if current.len() == 5 {
        check_permutation(&current)
    }

    for r in &remaining {
        let mut new_remaining = remaining.clone();
        let mut new_current = current.clone();
        new_remaining.remove(&r);
        new_current.push(*r);

        find_permutation( new_remaining, new_current);
    }
}

fn main() {
    let mut p = std::collections::HashSet::<i32>::new();
    p.insert(0);
    p.insert(1);
    p.insert(2);
    p.insert(3);
    p.insert(4);
    find_permutation(p, vec![]);

}

