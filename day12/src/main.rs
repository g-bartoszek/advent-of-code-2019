use regex::Regex;
use itertools::Itertools;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

type Vector = (i32, i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl Moon {
    fn advance(&mut self) {
        self.position = (self.position.0 + self.velocity.0,
                         self.position.1 + self.velocity.1,
                         self.position.2 + self.velocity.2)
    }

    fn apply_gravity(&mut self, other: &Moon) {
        if self.position.0 > other.position.0 {
            self.velocity.0 -= 1;
        }
        if self.position.0 < other.position.0 {
            self.velocity.0 += 1;
        }

        if self.position.1 > other.position.1 {
            self.velocity.1 -= 1;
        }
        if self.position.1 < other.position.1 {
            self.velocity.1 += 1;
        }

        if self.position.2 > other.position.2 {
            self.velocity.2 -= 1;
        }
        if self.position.2 < other.position.2 {
            self.velocity.2 += 1;
        }
    }

    fn energy(&self) -> i32 {
       (self.velocity.0.abs() +
        self.velocity.1.abs() +
        self.velocity.2.abs()) *
       (self.position.0.abs() +
        self.position.1.abs() +
        self.position.2.abs())
    }

}

fn compare(l: &[Moon; 4], r: &[Moon; 4]) -> bool {
    for i in 0..4 {
       if l[i].velocity.2 != r[i].velocity.2  || l[i].position.2 != r[i].position.2 {
            return false;
       }
    }
    true
}

fn main() {
    let input = "\
    <x=-1, y=0, z=2>\n\
    <x=2, y=-10, z=-7>\n\
    <x=4, y=-8, z=8>\n\
    <x=3, y=5, z=-1>\n\
    ";

    let input3 = "\
    <x=-8, y=-10, z=0>\n\
    <x=5, y=5, z=10>\n\
    <x=2, y=-7, z=3>\n\
    <x=9, y=-8, z=-3>\n\
    ";

    let input2 = "\
    <x=-8, y=-18, z=6>\n\
    <x=-11, y=-14, z=4>\n\
    <x=8, y=-3, z=-10>\n\
    <x=-2, y=-16, z=1>\n\
    ";

    let mut moonsv = input2.lines().map(|l| {
        let captures = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap().captures(l).unwrap();
        println!("{:?}", captures);
        Moon{
            position: (str::parse::<i32>(&captures[1]).unwrap(),
                       str::parse::<i32>(&captures[2]).unwrap(),
                       str::parse::<i32>(&captures[3]).unwrap()),
            velocity: (0,0,0) }
    }).collect::<Vec<_>>();

    let mut moons=  [Moon::default(); 4];
    moons.copy_from_slice(&moonsv);

    println!("{:?}", moons);

    let mut step = 0usize;
    let origin = moons.clone();

    let mut now = std::time::SystemTime::now();
    loop {
        if step % 10000000 == 0 {
            println!("Step: {} Elapsed: {:?}",
                     step,
                     now.elapsed().unwrap().as_secs(),
            );
        }

        step += 1;

      for m in 0..moons.len() {
          for o in 0..moons.len() {
              if o == m {
                  continue;
              }

              let other = moons[o].clone();
              moons[m].apply_gravity(&other);
          }
      }

      for m in &mut moons {
          m.advance();
      }
      if compare(&moons, &origin) {
          println!("!!! {}", step);
          break;
      }

    }

}
