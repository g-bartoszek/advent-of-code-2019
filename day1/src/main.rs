use std::io::{BufReader, BufRead};


fn get_fuel_amount(input: i32) -> i32 {
   let current =  input / 3 - 2;
   return if current > 0 {
      current + get_fuel_amount(current)
   } else {
      0
   }
}

fn main() {
   let input = BufReader::new(std::fs::File::open("input").unwrap());

   let r: i32 = input.lines()
       .filter_map(Result::ok)
       .map(|s| s.parse::<i32>())
       .filter_map(Result::ok)
       .map(|m| get_fuel_amount(m))
       .sum();

   println!("{}", r);
}
