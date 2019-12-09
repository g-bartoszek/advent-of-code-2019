use std::iter::FromIterator;
use std::time::Duration;
use std::cmp::Ordering;

mod interpreter;
use interpreter::Interpreter;


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

    let mut ampA = Interpreter::new(program.clone(), controller_to_a_rx, a_to_b.0);
    let mut ampB = Interpreter::new(program.clone(), a_to_b.1, b_to_c.0);
    let mut ampC = Interpreter::new(program.clone(), b_to_c.1, c_to_d.0);
    let mut ampD = Interpreter::new(program.clone(), c_to_d.1, d_to_e.0);
    let mut ampE = Interpreter::new(program.clone(), d_to_e.1, e_to_controller_tx);

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

