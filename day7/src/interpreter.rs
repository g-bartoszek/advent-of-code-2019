use std::fmt::Debug;

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
struct Input<'a> {
    input: &'a mut std::sync::mpsc::Receiver<i32>,
    result: Mode,
}
#[derive(Debug)]
struct Output<'a> {
    output: &'a mut std::sync::mpsc::Sender<i32>,
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

impl Instruction for Input<'_> {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        let input = self.input.recv().unwrap();
        *get_value(program, self.result) = input;
        *ip += 2
    }
}

impl Instruction for Output<'_> {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        let o = *get_value(program, self.what);
        self.output.send(o).unwrap();
        *ip += 2
    }
}

impl Instruction for JumpIfTrue {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        if *get_value(program, self.value) != 0 {
            *ip = *get_value(program, self.result) as usize;
        } else {
            *ip += 3
        }
    }
}

impl Instruction for JumpIfFalse {
    fn execute(&mut self, program: &mut Vec<i32>, ip: &mut usize) {
        if *get_value(program, self.value) == 0 {
            *ip = *get_value(program, self.result) as usize;
        } else {
            *ip += 3
        }
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

fn get_value(program: &mut Vec<i32>, mode: Mode) -> &mut i32 {
    match mode {
        Mode::Immediate(index) => &mut program[index],
        Mode::Position(index) => {
            let pos = program[index] as usize;
            &mut program[pos]
        },
    }
}

pub struct Interpreter {
    input: std::sync::mpsc::Receiver<i32>,
    output: std::sync::mpsc::Sender<i32>,
    program: Vec<i32>,
    ip: usize
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Status {
    Run,
    Halt,
}

fn print_state(s: &Vec<i32>, ip: usize) {
    print!("STATE: ");
    for i in 0..s.len() {
        if i == ip {
            print!("[{}], ", s[i]);
        } else {
            print!("{}, ", s[i]);
        }
    }
    println!();
}

impl<'a> Interpreter {

    pub fn new(program: Vec<i32>, input: std::sync::mpsc::Receiver<i32>, output: std::sync::mpsc::Sender<i32>) -> Self {
        Self{input, output, program, ip: 0usize}
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

        {
            let mut instr: Option<Box<Instruction>> = match opcode {
                1 => Some(Box::new(Add { lhs: mode1, rhs: mode2, result: Mode::Position(ip + 3) })),
                2 => Some(Box::new(Mul { lhs: mode1, rhs: mode2, result: Mode::Position(ip + 3) })),
                3 => Some(Box::new(Input { input: &mut self.input, result: Mode::Position(ip + 1) })),
                4 => Some(Box::new(Output { what: mode1, output: &mut self.output })),
                5 => Some(Box::new(JumpIfTrue { value: mode1, result: mode2 })),
                6 => Some(Box::new(JumpIfFalse { value: mode1, result: mode2 })),
                7 => Some(Box::new(LessThan { lhs: mode1, rhs: mode2, result: Mode::Position(ip + 3) })),
                8 => Some(Box::new(Equals { lhs: mode1, rhs: mode2, result: Mode::Position(ip + 3) })),
                99 => None,
                _ => panic!("Unrecognized instruction!")
            };

            if let Some(mut i) = instr {
                //println!("IP: {} INSTR: {:?}",self.ip, i);
                i.execute(&mut self.program, &mut self.ip);
                //print_state(&self.program, self.ip);
                //println!("\n\n\n\n\n")
                Status::Run
            } else {
                Status::Halt
            }
        }
    }

    pub fn process(&mut self) {
        loop {
            if let Status::Halt = self.parse_instruction() {
                return;
            }
        }
    }
}
