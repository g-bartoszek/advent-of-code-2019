mod interpreter;
use interpreter::*;

fn main() {
    let input = vec![109,424,203,1,21102,11,1,0,1106,0,282,21101,18,0,0,1106,0,259,2101,0,1,221,203,1,21102,31,1,0,1106,0,282,21102,1,38,0,1105,1,259,20102,1,23,2,22101,0,1,3,21101,0,1,1,21101,0,57,0,1106,0,303,1202,1,1,222,21001,221,0,3,20102,1,221,2,21102,259,1,1,21101,80,0,0,1105,1,225,21102,1,149,2,21101,0,91,0,1105,1,303,1202,1,1,223,21002,222,1,4,21102,259,1,3,21102,225,1,2,21102,225,1,1,21101,118,0,0,1105,1,225,20102,1,222,3,21101,0,127,2,21102,133,1,0,1105,1,303,21202,1,-1,1,22001,223,1,1,21102,1,148,0,1106,0,259,1201,1,0,223,21001,221,0,4,21002,222,1,3,21102,14,1,2,1001,132,-2,224,1002,224,2,224,1001,224,3,224,1002,132,-1,132,1,224,132,224,21001,224,1,1,21101,195,0,0,106,0,108,20207,1,223,2,20102,1,23,1,21101,0,-1,3,21102,214,1,0,1106,0,303,22101,1,1,1,204,1,99,0,0,0,0,109,5,1202,-4,1,249,22102,1,-3,1,21201,-2,0,2,21201,-1,0,3,21102,1,250,0,1105,1,225,22102,1,1,-4,109,-5,2106,0,0,109,3,22107,0,-2,-1,21202,-1,2,-1,21201,-1,-1,-1,22202,-1,-2,-2,109,-3,2105,1,0,109,3,21207,-2,0,-1,1206,-1,294,104,0,99,21202,-2,1,-2,109,-3,2106,0,0,109,5,22207,-3,-4,-1,1206,-1,346,22201,-4,-3,-4,21202,-3,-1,-1,22201,-4,-1,2,21202,2,-1,-1,22201,-4,-1,1,22101,0,-2,3,21101,343,0,0,1106,0,303,1106,0,415,22207,-2,-3,-1,1206,-1,387,22201,-3,-2,-3,21202,-2,-1,-1,22201,-3,-1,3,21202,3,-1,-1,22201,-3,-1,2,22101,0,-4,1,21102,1,384,0,1106,0,303,1105,1,415,21202,-4,-1,-4,22201,-4,-3,-4,22202,-3,-2,-2,22202,-2,-4,-4,22202,-3,-2,-3,21202,-4,-1,-2,22201,-3,-2,1,22102,1,1,-4,109,-5,2106,0,0
    ];

    let check_pos = |x,y| {
        let (mut input_tx, input_rx) = std::sync::mpsc::sync_channel::<interpreter::IntCode>(1);
        let (output_tx, mut output_rx) = std::sync::mpsc::channel::<interpreter::IntCode>();


        let mut interpreter = Interpreter::new(Vec::from(input.clone()), input_rx, output_tx);

        std::thread::spawn(move || interpreter.process());
        input_tx.send(x).unwrap();
        input_tx.send(y).unwrap();

        output_rx.recv().unwrap()
    };


    let mut sum = 0;

    let mut y = 9;
    let mut x = 8;
    let mut last = 0;

    loop {
        println!("{} {}", x, y);
        let v = check_pos(x,y);

        if v == 0 && last == 1 {

            if (x-1) >= 99 && check_pos(x-100, y+99) == 1 {
                println!("SUCCESS: {} {} {}", x-100 ,y, (x-100) * 10000 + y);
                return;
            }

            y += 1;
            x = x-3;

        } else {
            x+=1;
        }
        last = v;
    }


    println!("Sum: {}", sum);
}