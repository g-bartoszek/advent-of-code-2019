use std::io::BufRead;

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

fn main() {
    let input = std::io::BufReader::new(std::fs::File::open("input_t2").unwrap());
    let mut map = Map::new();
    let mut keys = Vec::<(char, Point)>::new();
    let mut doors = std::collections::HashMap::<char, Point>::new();
    let mut start = (0,0);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.unwrap().chars().enumerate() {
            print!("{}", c);
            let x = x as i32 + 1;
            let y = y as i32 + 1;
            map.insert((x,y), match c {
                '#' => Field::Wall,
                '.' => Field::Empty,
                '@' => {
                    start = (x,y);
                    Field::Empty
                },
                f if f.is_uppercase() => {
                    doors.insert(f, (x,y));
                    (Field::Door(f))
                },
                f if f.is_lowercase() => {
                    keys.push((f, (x,y)));
                    (Field::Key(f))
                },
                _ => panic!("Unrecognized field {}", c)
            });
        }
        println!();
    }

    let mut min = std::usize::MAX;
    search(&map, start, keys.clone(), &doors);

}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    remaining_keys: Vec<(char, Point)>,
    distance: usize,
    position: Point
}

fn search(map: &Map,
          start: Point,
          remaining_keys: Vec<(char, Point)>,
          doors: &std::collections::HashMap<char, Point>) {

    let mut open = std::collections::HashSet::<State>::new();

    open.insert(State{remaining_keys, distance: 0, position: start});

    loop {
        println!("{:?}", open.len());
        let current = open.iter().min_by(|l,r| l.distance.cmp(&r.distance)).unwrap().clone();


        if current.remaining_keys.is_empty() {
            println!("FOUND: {}", current.distance);
            return;
        }

        open.remove(&current);
        for (key, position) in &current.remaining_keys {
            if let Some(distance) = find_path(map, current.position, *position, &current.remaining_keys){
                let new_keys = current.remaining_keys.iter().filter(|(k, _)| *k != *key).map(|k| *k).collect::<Vec<_>>();
                open.insert(State{remaining_keys: new_keys, distance: current.distance + distance, position: *position});
            }
        }
    }

}

fn find_path(map: &Map, start: Point, target: Point, remaining_keys: &Vec<(char, Point)>) -> Option<usize> {
    let mut examined = std::collections::HashSet::<Point>::new();
    let mut open = std::collections::HashSet::<(Point,usize)>::new();

    open.insert((start, 0));

    loop {
        if open.is_empty() {
            return None;
        }

        let current = *open.iter().min_by(|((_,_), l), ((_,_), r)| l.cmp(r)).unwrap();
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

            if distance > 700 {
                continue;
            }

            //println!("Neighbor: {:?} {:?}", neighbor, map.get(&neighbor).unwrap_or(&Field::Undefined));
            if *neighbor == target {
                return Some(distance);
            } else if !examined.contains(neighbor) && {
                let field = map.get(&neighbor).unwrap_or(&Field::Undefined);
                if let Field::Key(_) = field {
                    true
                } else if let Field::Door(d) = field {
                    remaining_keys.iter().find(|(k, _)| *k == d.to_lowercase().next().unwrap()).is_none()
                } else {
                    *field == Field::Empty
                }
            } {
                //println!("Added");
                open.insert((*neighbor, distance));
            }
        }
    }

}
