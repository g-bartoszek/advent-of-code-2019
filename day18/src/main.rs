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
type Map = std::collections::BTreeMap<Point, Field>;

fn main() {
    let input = std::io::BufReader::new(std::fs::File::open("input_t2").unwrap());
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
    search(map.clone(), start, keys.clone(), &doors, 0, &mut min);

    println!("MIN: {}", min);
}

fn search(map: Map,
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

    let mut sorted_keys = remaining_keys.iter().collect::<Vec<(&char, &Point)>>();
    sorted_keys.sort_by(|(_,l), (_,r)| {
        let l_dist = (start.0 - l.0).abs() + (start.1 - l.1).abs();
        let r_dist = (start.0 - r.0).abs() + (start.1 - r.1).abs();
        l_dist.cmp(&r_dist)
    });

    for k in sorted_keys.iter() {

        //println!("KEY: {:?}", k);

        if let Some(d) = find_path(&map, start, *k.1) {


            if *current_min < (d+distance) {
                //println!("DROPPED, min is {}", *current_min);
                continue;
            }


            let mut new_map = map.clone();
            *new_map.get_mut(&k.1).unwrap() = Field::Empty;
            if let Some(doors) = doors.get(&k.0.to_uppercase().next().unwrap()) {
                *new_map.get_mut(&doors).unwrap() = Field::Empty;
            }
            let mut new_keys = remaining_keys.clone();
            new_keys.remove(&k.0);

            search(new_map, *k.1, new_keys, doors, distance + d, current_min);
        }
    }

}

fn find_path(map: &Map, start: Point, target: Point) -> Option<usize> {
    let mut examined = std::collections::BTreeSet::<Point>::new();
    let mut open = std::collections::BTreeSet::<(Point,usize)>::new();

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
            //println!("Neighbor: {:?} {:?}", neighbor, map.get(&neighbor).unwrap_or(&Field::Undefined));
            if *neighbor == target {
                return Some(distance);
            } else if !examined.contains(neighbor) && *map.get(&neighbor).unwrap_or(&Field::Undefined) == Field::Empty {
                //println!("Added");
                open.insert((*neighbor, distance));
            }
        }
    }

}
