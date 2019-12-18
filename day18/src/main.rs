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
    let input = std::io::BufReader::new(std::fs::File::open("input").unwrap());
    let mut map = Map::new();
    let mut keys = std::collections::HashMap::<char, Point>::new();
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
                    keys.insert(f, (x,y));
                    (Field::Key(f))
                },
                _ => panic!("Unrecognized field {}", c)
            });
        }
        println!();
    }

    let mut min = std::usize::MAX;
    search(&map, start, keys.clone(), &doors, 0, &mut min);

    println!("MIN: {}", min);
}

fn search(map: &Map,
          start: Point,
          remaining_keys: std::collections::HashMap<char, Point>,
          doors: &std::collections::HashMap<char, Point>,
          distance: usize,
          current_min: &mut usize) {

    if remaining_keys.is_empty() {
        println!("KEYS CLEARED: {}", distance);
        *current_min = distance;
        return;
    }

    let mut sorted_keys = remaining_keys.iter().map(|k| {
        (k, find_path(&map, start, *k.1, &remaining_keys))
    }).filter(|(k,p)| p.is_some()).collect::<Vec<_>>();

    sorted_keys.sort_by(|l,r| l.1.cmp(&r.1));

    for k in sorted_keys.iter() {

        //println!("KEY: {:?}", k);

        let d = k.1.unwrap();

        if *current_min < (d+distance) {
            //println!("DROPPED, min is {}", *current_min);
            continue;
        }

        let mut new_keys = remaining_keys.clone();
        new_keys.remove(&(k.0).0);

        search(map, *(k.0).1, new_keys, doors, distance + d, current_min);
    }

}

fn find_path(map: &Map, start: Point, target: Point, remaining_keys: &std::collections::HashMap<char, Point>) -> Option<usize> {
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
                    !remaining_keys.contains_key(&d.to_lowercase().next().unwrap())
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
