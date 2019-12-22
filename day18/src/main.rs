use std::io::BufRead;
use std::cmp::{Ordering, Reverse};
use std::thread::current;

#[derive(PartialEq, Debug, Clone)]
enum Field {
    Wall,
    Empty,
    Key(char),
    Door(char),
    Start,
    Undefined
}

type Point = (i32,i32);
type Map = std::collections::HashMap<Point, Field>;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct State(Point,usize,std::collections::BTreeSet<char>);

impl State {
   fn weight(&self) -> usize {
       self.1
   }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight().partial_cmp(&other.weight())
    }
}

fn main() {
    let input = std::io::BufReader::new(std::fs::File::open("input").unwrap());
    let mut map = Map::new();
    let mut places = Vec::<Point>::new();
    let mut start = (0,0);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.unwrap().chars().enumerate() {
            print!("{}", c);
            let x = x as i32;
            let y = y as i32;
            map.insert((x,y), match c {
                '#' => Field::Wall,
                '.' => Field::Empty,
                '@' => {
                    places.push((x,y));
                    start = (x,y);
                    Field::Empty
                },
                f if f.is_uppercase() => {
                    places.push((x,y));
                    (Field::Door(f))
                },
                f if f.is_lowercase() => {
                    places.push((x,y));
                    (Field::Key(f))
                },
                _ => panic!("Unrecognized field {}", c)
            });
        }
        println!();
    }

    let number_of_keys = map.iter().filter(|(_, f)| if let Field::Key(_) = f {
        true
    } else {
        false
    }).count();

    println!("Keys to find: {}", number_of_keys);

    let mut paths = std::collections::HashMap::<Point, Vec<(Point, usize)>>::new();

    for from in &places {
        for to in &places {
            if from == to {
                continue;
            }

            if let Some(d) = find_path(&map, *from, *to) {
                paths.entry(*from).or_insert(Vec::<(Point, usize)>::new()).push((*to, d));
            }
        }
    }

    for p in &paths {
        println!("{:?}", p);
    }
    let mut open = std::collections::BinaryHeap::<Reverse<State>>::new();

    open.push(Reverse(State(start, 0, std::collections::BTreeSet::<char>::new())));

    loop {
        //println!("{:?}", open);
        //std::io::stdin().read_line(&mut String::new());

        let current = open.pop().unwrap().0;

        //println!("Current {:?}", current);
        if current.2.len() == number_of_keys {
            println!("SUCCESS: {} ", current.1);
            return;
        }

        let current_pos = current.0;

        for path in &paths[&current.0] {
            let mut new_keys = current.2.clone();
            match map[&path.0] {
                Field::Door(d) => {
                    if !current.2.contains(&d.to_lowercase().next().unwrap()) {
                        //println!("Skipping path because of closed doors {:?}", path);
                        continue;
                    }
                },
                Field::Key(k) => {
                    new_keys.insert(k);
                }
                _ => {}
            }

            //let mut visited = current.3.clone();
            //visited.push(path.0);

            //println!("Added path {:?}", path);
            let new_state = Reverse(State(path.0, current.1 + path.1, new_keys));
            if open.iter().find(|&e| *e == new_state).is_none() {
                open.push(new_state);
            }
        }
    }
}

fn find_path(map: &Map, start: Point, target: Point) -> Option<usize> {
    let mut examined = std::collections::HashSet::<Point>::new();
    let mut open = std::collections::HashSet::<(Point,usize)>::new();

    open.insert((start, 0));

    loop {
        if open.is_empty() {
            return None;
        }

        let current = *open.iter().min_by(|((lx,ly), l), ((rx,ry), r)| {
            let l_dist = (target.0 - *lx).abs() + (target.1 - *ly).abs();
            let r_dist = (target.0 - *rx).abs() + (target.1 - *ry).abs();
            (l + l_dist as usize).cmp(&(r + r_dist as usize))
        }).unwrap();
        open.remove(&current);

        //println!("\n\nCurrent: {:?}", current);

        examined.insert(current.0);

        let current_pos = current.0;

        for neighbor in [
            (current_pos.0 + 1, current_pos.1),
            (current_pos.0 - 1, current_pos.1),
            (current_pos.0, current_pos.1 + 1),
            (current_pos.0, current_pos.1 - 1),
        ].iter() {
            let distance = current.1 + 1;

            //println!("Neighbor: {:?} {:?}", neighbor, map.get(&neighbor).unwrap_or(&Field::Undefined));
            if *neighbor == target {
                return Some(distance);
            } else if !examined.contains(neighbor) {
                let field = map.get(&neighbor).unwrap_or(&Field::Undefined);
                if *field == Field::Empty {
                    open.insert((*neighbor, distance));
                }
            }
        }
    }

}
