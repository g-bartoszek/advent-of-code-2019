
fn search(index: usize, current: u32, previous: u32) -> usize {
    let mut result = 0usize;

    for digit in previous..10 {
        let mut new_current = current;
        new_current += digit * 10_u32.pow(5u32-index as u32);

        if new_current > 784965 {
            continue;
        }

        if index < 5 {
            result += search(index + 1, new_current, digit);
        } else {

            if new_current < 240298  {
                continue
            }

            let digits = new_current.to_string().chars().collect::<Vec<_>>();
            for i in 0..(digits.len() - 1) {
                if digits[i] == digits[i+1] {
                    if i > 0 && digits[i-1] == digits[i] {
                            continue;
                    }
                    if i < digits.len() - 2 && digits[i+2] == digits[i] {
                            continue;
                    }

                    println!("Found: {:?}", new_current);
                    result += 1;
                    break;
                }

            }
        }
    }

    result
}

fn main() {

    println!("Result: {}", search(0, 0, 0));

}
