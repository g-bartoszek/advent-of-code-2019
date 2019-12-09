use std::fmt::Debug;
use std::iter::FromIterator;
use std::time::Duration;
use std::cmp::Ordering;

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

struct Parser{
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

impl<'a> Parser {

    fn new(program: Vec<i32>, input: std::sync::mpsc::Receiver<i32>, output: std::sync::mpsc::Sender<i32>) -> Self {
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

    fn process(&mut self) {
        loop {
            if let Status::Halt = self.parse_instruction() {
                return;
            }
        }
    }
}

fn check_permutation(p: &Vec<i32>, r: &mut Vec<i32>) {
    let program = vec![3,8,1001,8,10,8,105,1,0,0,21,38,55,64,81,106,187,268,349,430,99999,3,9,101,2,9,9,1002,9,2,9,101,5,9,9,4,9,99,3,9,102,2,9,9,101,3,9,9,1002,9,4,9,4,9,99,3,9,102,2,9,9,4,9,99,3,9,1002,9,5,9,1001,9,4,9,102,4,9,9,4,9,99,3,9,102,2,9,9,1001,9,5,9,102,3,9,9,1001,9,4,9,102,5,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,99];
    println!("Permutation: {:?}", p);

    let (controller_to_a_tx, controller_to_a_rx) = std::sync::mpsc::channel::<i32>();
    controller_to_a_tx.send(p[0]);
    controller_to_a_tx.send(0);
    let a_to_b = std::sync::mpsc::channel::<i32>();
    a_to_b.0.send(p[1]);
    let b_to_c = std::sync::mpsc::channel::<i32>();
    b_to_c.0.send(p[2]);
    let c_to_d = std::sync::mpsc::channel::<i32>();
    c_to_d.0.send(p[3]);
    let d_to_e = std::sync::mpsc::channel::<i32>();
    d_to_e.0.send(p[4]);
    let (e_to_controller_tx, e_to_controller_rx) = std::sync::mpsc::channel::<i32>();

    let mut ampA = Parser::new(program.clone(), controller_to_a_rx, a_to_b.0);
    let mut ampB = Parser::new(program.clone(), a_to_b.1, b_to_c.0);
    let mut ampC = Parser::new(program.clone(), b_to_c.1, c_to_d.0);
    let mut ampD = Parser::new(program.clone(), c_to_d.1, d_to_e.0);
    let mut ampE = Parser::new(program.clone(), d_to_e.1, e_to_controller_tx);

    let t1 = std::thread::spawn(move || ampA.process());
    let t2 = std::thread::spawn(move || ampB.process());
    let t3 = std::thread::spawn(move || ampC.process());
    let t4 = std::thread::spawn(move || ampD.process());
    let t5 = std::thread::spawn(move || ampE.process());

    let run = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let rh = run.clone();
    let t6 = std::thread::spawn(move || {
        let mut last_result = 0;
        while run.load(std::sync::atomic::Ordering::SeqCst) {
            if let Ok(r) = e_to_controller_rx.recv_timeout(Duration::from_millis(100)) {
                last_result = r;
                controller_to_a_tx.send(last_result);
            }
        }

        return last_result;
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
    t5.join().unwrap();

    rh.store(false, std::sync::atomic::Ordering::SeqCst);

    r.push(t6.join().unwrap());
}

fn find_permutation(remaining: std::collections::HashSet<i32>, current: Vec<i32>, res: &mut Vec<i32>) {
    if current.len() == 5 {
        check_permutation(&current, res)
    }

    for r in &remaining {
        let mut new_remaining = remaining.clone();
        let mut new_current = current.clone();
        new_remaining.remove(&r);
        new_current.push(*r);

        find_permutation( new_remaining, new_current, res);
    }
}

fn main() {
    let mut p = std::collections::HashSet::<i32>::new();
    let mut r = Vec::<i32>::new();
    p.insert(5);
    p.insert(6);
    p.insert(7);
    p.insert(8);
    p.insert(9);
    find_permutation(p, vec![], &mut r);

    print!("{:?}", r.iter().max());
}

