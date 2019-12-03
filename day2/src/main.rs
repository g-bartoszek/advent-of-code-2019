
fn process(noun: i32, verb: i32) -> i32 {
    let mut input: [i32; 121] = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,6,19,23,2,23,6,27,2,6,27,31,2,13,31,35,1,10,35,39,2,39,13,43,1,43,13,47,1,6,47,51,1,10,51,55,2,55,6,59,1,5,59,63,2,9,63,67,1,6,67,71,2,9,71,75,1,6,75,79,2,79,13,83,1,83,10,87,1,13,87,91,1,91,10,95,2,9,95,99,1,5,99,103,2,10,103,107,1,107,2,111,1,111,5,0,99,2,14,0,0];
    input[1] = noun;
    input[2] = verb;

    for instruction in (0..input.len()).step_by(4) {
        match input[instruction] {
            1 => { input[input[instruction+3] as usize] = input[input[instruction+1] as usize] + input[input[instruction+2] as usize]}
            2 => { input[input[instruction+3] as usize] = input[input[instruction+1] as usize] * input[input[instruction+2] as usize]}
            99 => { return input[0];}
            _ => {}
        }

    }

    return 0
}

fn main() {
    for n in 0..99 {
        for v in 0..99 {
            if process(n, v) == 19690720 {
                println!("Got it: {}", 100 * n + v);
                return;
            }
        }
    }

}
