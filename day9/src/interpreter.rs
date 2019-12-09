use std::fmt::Debug;

pub type IntCode = i64;
type Program = Vec<IntCode>;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Mode {
    Position(usize),
    Immediate(usize),
    Relative(usize)
}

trait Instruction: Debug {
    fn execute(&mut self, interpreter: &mut Interpreter);
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
    result: Mode,
}
#[derive(Debug)]
struct Output {
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

#[derive(Debug)]
struct AdjustRelativeBase {
    value: Mode,
}

impl Instruction for Add {
    fn execute(&mut self, interpreter: &mut Interpreter) {
        *interpreter.get_value(self.result) = *interpreter.get_value(self.lhs) +  *interpreter.get_value(self.rhs);
        interpreter.ip += 4
    }
}

impl Instruction for Mul {
    fn execute(&mut self, interpreter: &mut Interpreter) {
        *interpreter.get_value(self.result) = *interpreter.get_value(self.lhs) *  *interpreter.get_value(self.rhs);
        interpreter.ip += 4
    }
}

impl Instruction for Input {
    fn execute(&mut self, interpreter: &mut Interpreter) {
        let input = interpreter.input.recv().unwrap();
        *interpreter.get_value(self.result) = input;
        interpreter.ip += 2
    }
}

impl Instruction for Output {
    fn execute(&mut self, interpreter: &mut Interpreter) {
        let o = *interpreter.get_value(self.what);
        println!("Output: {}", o);
        interpreter.output.send(o).unwrap();
        interpreter.ip += 2
    }
}

impl Instruction for JumpIfTrue {
    fn execute(&mut self, interpreter: &mut Interpreter) {
        if *interpreter.get_value(self.value) != 0 {
            interpreter.ip = *interpreter.get_value(self.result) as usize;
        } else {
            interpreter.ip += 3
        }
    }
}

impl Instruction for JumpIfFalse {
    fn execute(&mut self, interpreter: &mut Interpreter) {
        if *interpreter.get_value(self.value) == 0 {
            interpreter.ip = *interpreter.get_value(self.result) as usize;
        } else {
            interpreter.ip += 3
        }
    }
}

impl Instruction for LessThan {
    fn execute(&mut self, interpreter: &mut Interpreter) {
        if *interpreter.get_value(self.lhs) < *interpreter.get_value(self.rhs) {
            *interpreter.get_value(self.result) = 1;
        } else {
            *interpreter.get_value(self.result) = 0;
        }
        interpreter.ip += 4
    }
}

impl Instruction for Equals {
    fn execute(&mut self, interpreter: &mut Interpreter) {
        if *interpreter.get_value(self.lhs) == *interpreter.get_value(self.rhs) {
            *interpreter.get_value(self.result) = 1;
        } else {
            *interpreter.get_value(self.result) = 0;
        }
        interpreter.ip += 4
    }
}

impl Instruction for AdjustRelativeBase {
    fn execute(&mut self, interpreter: &mut Interpreter) {
        interpreter.relative_base += *interpreter.get_value(self.value);
        interpreter.ip += 2
    }
}


pub struct Interpreter {
    input: std::sync::mpsc::Receiver<IntCode>,
    output: std::sync::mpsc::Sender<IntCode>,
    program: Vec<IntCode>,
    ip: usize,
    relative_base: IntCode
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Status {
    Run,
    Halt,
}

fn print_state(s: &Vec<IntCode>, ip: usize) {
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

    pub fn new(program: Vec<IntCode>, input: std::sync::mpsc::Receiver<IntCode>, output: std::sync::mpsc::Sender<IntCode>) -> Self {
        Self{input, output, program, ip: 0usize, relative_base: 0}
    }

    fn get_mode(value: IntCode, location: usize) -> Mode {
        match value {
            2 => Mode::Relative(location),
            1 => Mode::Immediate(location),
            0 => Mode::Position(location),
            _ => panic!("Unsupported mode!")
        }
    }

    fn parse_instruction(&'a mut self) -> Status {
        let ip = self.ip;
        let mut opcode = self.program[ip];
        let mode3 = Self::get_mode(opcode / 10000, ip+3);
        opcode = opcode % 10000;
        let mode2 = Self::get_mode(opcode / 1000, ip+2);
        opcode = opcode % 1000;
        let mode1 = Self::get_mode(opcode / 100, ip+1);
        opcode = opcode % 100;

        {
            let mut instr: Option<Box<dyn Instruction>> = match opcode {
                1 => Some(Box::new(Add { lhs: mode1, rhs: mode2, result: mode3 })),
                2 => Some(Box::new(Mul { lhs: mode1, rhs: mode2, result: mode3 })),
                3 => Some(Box::new(Input { result: mode1 })),
                4 => Some(Box::new(Output { what: mode1 })),
                5 => Some(Box::new(JumpIfTrue { value: mode1, result: mode2 })),
                6 => Some(Box::new(JumpIfFalse { value: mode1, result: mode2 })),
                7 => Some(Box::new(LessThan { lhs: mode1, rhs: mode2, result: mode3 })),
                8 => Some(Box::new(Equals { lhs: mode1, rhs: mode2, result: mode3 })),
                9 => Some(Box::new(AdjustRelativeBase { value: mode1 })),
                99 => None,
                _ => panic!("Unrecognized instruction!")
            };

            if let Some(mut i) = instr {
           //     println!("\n\nOPCODE: {} IP: {} INSTR: {:?} RBASE: {}", self.program[ip], self.ip, i, self.relative_base);
                i.execute(self);
            //    print_state(&self.program, self.ip);
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

    fn get_value(&mut self, mode: Mode) -> &mut IntCode {
        let r = match mode {
            Mode::Immediate(index) => &mut self.program[index],
            Mode::Position(index) => {
                let pos = self.program[index] ;
                if pos as usize >= self.program.len() {
              //      println!("Resize to: {}", pos+1);
                    self.program.resize((pos + 1) as usize, 0);
                }
                &mut self.program[pos as usize]
            },
            Mode::Relative(index) => {
                let pos = self.program[index] + self.relative_base;
                if pos as usize >= self.program.len() {
               //     println!("Resize to: {}", pos+1);
                    self.program.resize((pos + 1) as usize, 0);
                }
                &mut self.program[pos as usize]
            },
        };
     //   println!("Got value: {}", r);
        r
    }
}
