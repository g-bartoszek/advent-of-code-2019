use regex::Regex;
use itertools::Itertools;

type Vector = (i32, i32, i32);

#[derive(Debug, Clone, Copy)]
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

fn main() {
    let input = "\
    <x=-1, y=0, z=2>\n\
    <x=2, y=-10, z=-7>\n\
    <x=4, y=-8, z=8>\n\
    <x=3, y=5, z=-1>\n\
    ";

    let input = "\
    <x=-8, y=-18, z=6>\n\
    <x=-11, y=-14, z=4>\n\
    <x=8, y=-3, z=-10>\n\
    <x=-2, y=-16, z=1>\n\
    ";

    let mut moons = input.lines().map(|l| {
        let captures = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap().captures(l).unwrap();
        println!("{:?}", captures);
        Moon{
            position: (str::parse::<i32>(&captures[1]).unwrap(),
                       str::parse::<i32>(&captures[2]).unwrap(),
                       str::parse::<i32>(&captures[3]).unwrap()),
            velocity: (0,0,0) }
    }).collect::<Vec<_>>();

    println!("{:?}", moons);

    for s in 1..1001 {

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

      println!("{} {:?} {}",s,  moons, moons.iter().map(Moon::energy).sum::<i32>());
    }


}
