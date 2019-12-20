use std::io::{BufRead, Read};
use std::cmp::Ordering;

#[derive(PartialEq, Debug, Clone)]
enum Field {
    Wall,
    Empty,
    Space,
    Undefined,
}

type Point = (i32, i32);
type Map = std::collections::HashMap<Point, Field>;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum PortalType {
    Outer,
    Inner,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Portal {
    position: Point,
    portal_type: PortalType,
}

fn get_portal_type(position: Point, max_x: i32, max_y: i32) -> Portal {
    let portal_type = match position {
        (x, _) if x == 2 => PortalType::Outer,
        (x, _) if x == max_x - 2 => PortalType::Outer,
        (_, y) if y == 2 => PortalType::Outer,
        (_, y) if y == max_y - 2 => PortalType::Outer,
        _ => PortalType::Inner
    };

    Portal { position, portal_type }
}

fn main() {
    let input = std::io::BufReader::new(std::fs::File::open("input").unwrap());
    let mut map = Map::new();
    let mut keys = Vec::<(char, Point)>::new();
    let mut letters = std::collections::HashMap::<Point, char>::new();
    let mut portals = std::collections::HashMap::<[char; 2], Portal>::new();
    let mut start = (0, 0);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.unwrap().chars().enumerate() {
            print!("{}", c);
            let x = x as i32;
            let y = y as i32;
            map.insert((x, y), match c {
                '#' => Field::Wall,
                '.' => Field::Empty,
                ' ' => Field::Space,
                _ => {
                    letters.insert((x, y), c);
                    Field::Space
                }
            });
        }
        println!();
    }

    let max_x = *map.iter().map(|((x, _), _)| x).max().unwrap();
    let max_y = *map.iter().map(|((_, y), _)| y).max().unwrap();

    for (position, field) in &map {
        if *field == Field::Empty {
            if *map.get(&(position.0 - 1, position.1)).unwrap_or(&Field::Undefined) == Field::Space {
                let mut name = [letters[&(position.0 - 1, position.1)], letters[&(position.0 - 2, position.1)]];
                if portals.contains_key(&name) {
                    name.reverse();
                }

                portals.insert(name, get_portal_type(*position, max_x, max_y));
            }

            if *map.get(&(position.0 + 1, position.1)).unwrap_or(&Field::Undefined) == Field::Space {
                let mut name = [letters[&(position.0 + 1, position.1)], letters[&(position.0 + 2, position.1)]];
                if portals.contains_key(&name) {
                    name.reverse();
                }
                portals.insert(name, get_portal_type(*position, max_x, max_y));
            }

            if *map.get(&(position.0, position.1 - 1)).unwrap_or(&Field::Undefined) == Field::Space {
                let mut name = [letters[&(position.0, position.1 - 1)], letters[&(position.0, position.1 - 2)]];
                if portals.contains_key(&name) {
                    name.reverse();
                }
                portals.insert(name, get_portal_type(*position, max_x, max_y));
            }

            if *map.get(&(position.0, position.1 + 1)).unwrap_or(&Field::Undefined) == Field::Space {
                let mut name = [letters[&(position.0, position.1 + 1)], letters[&(position.0, position.1 + 2)]];
                if portals.contains_key(&name) {
                    name.reverse();
                }
                portals.insert(name, get_portal_type(*position, max_x, max_y));
            }
        }
    }

    println!("{:?}", portals);
    let mut paths = std::collections::HashMap::<Point, Vec<(Portal, usize)>>::new();

    for from in &portals {
        for to in &portals {
            if *from.0 == ['Z', 'Z'] || from == to || *to.0 == ['A', 'A'] {
                continue;
            }

            let from_position = from.1.position;
            let to_position = to.1.position;

            if let Some(d) = find_path(&map, from_position, to_position) {
                println!("Path from {:?} to {:?} is {:?}", from, to, d);
                if *to.0 == ['Z', 'Z'] {
                    //println!("It's target");
                    paths.entry(from_position).or_insert(Vec::<(Portal, usize)>::new()).push((Portal { position: to_position, portal_type: to.1.portal_type }, d));
                } else {
                    let mut target_portal = *to.0;
                    target_portal.reverse();
                    let destination = portals[&target_portal].position;
                    //println!("Teleport to {:?} {:?}", target_portal,  to);
                    paths.entry(from_position).or_insert(Vec::<(Portal, usize)>::new()).push((Portal { position: destination, portal_type: to.1.portal_type }, d + 1));
                }
            }
        }
    }
    println!("{:?}", paths);

    /////////////SEARCH
    //let mut examined = std::collections::HashSet::<(Point,usize)>::new();
    let mut open = std::collections::HashSet::<(Point, usize, usize)>::new();

    open.insert((portals[&['A', 'A']].position, 0, 0));

    loop {
        for o in &open {
            println!("{:?}", o);
        }

       //std::io::stdin().read_line(&mut String::new());


        if open.is_empty() {
            panic!("PATH NOT FOUND");
        }

        let current = *open.iter().min_by(|((lx, ly), l, l_level), ((rx, ry), r, r_level)| {
            (l).cmp(&(r))
        }).unwrap();

        open.remove(&current);

        //examined.insert((current.0, current.2));

        println!("Current: {:?}", current);

        let current_pos = current.0;
        if current_pos == portals[&['Z', 'Z']].position {
            println!("DISTANCE: {}", current.1);
            return;
        }

        for neighbor in &paths[&current_pos] {
            if current.2 == 0 && neighbor.0.portal_type == PortalType::Outer && neighbor.0.position != portals[&['Z', 'Z']].position {
                println!("Skipped because outer and level 0");
                continue;
            }

            if current.2 > 0 && neighbor.0.position == portals[&['Z', 'Z']].position {
                println!("Skipped because it's ZZ and we're on the inner level");
                continue;
            }

            let distance = current.1 + neighbor.1;

            let level = match neighbor.0.portal_type {
              PortalType::Inner => {
                  println!("LEVEL UP");
                  current.2 + 1
              },
              PortalType::Outer => {
                  println!("LEVEL DOWN");
                  current.2 - 1
              },
            };

          //  if !examined.contains(&(neighbor.0.position, current.2)) {
                open.insert((neighbor.0.position, distance, level));
         //   }
        }
    }
}

fn neighbors(point: Point) -> [Point; 4] {
    [
        (point.0 + 1, point.1),
        (point.0 - 1, point.1),
        (point.0, point.1 + 1),
        (point.0, point.1 - 1),
    ]
}


fn find_path(map: &Map, start: Point, target: Point) -> Option<usize> {
    let mut examined = std::collections::HashSet::<Point>::new();
    let mut open = std::collections::HashSet::<(Point, usize)>::new();

    open.insert((start, 0));

    loop {
        if open.is_empty() {
            return None;
        }

        let current = *open.iter().min_by(|((lx, ly), l), ((rx, ry), r)| {
            let l_dist = (target.0 - *lx).abs() + (target.1 - *ly).abs();
            let r_dist = (target.0 - *rx).abs() + (target.1 - *ry).abs();
            (l + l_dist as usize).cmp(&(r + r_dist as usize))
        }).unwrap();
        open.remove(&current);

        examined.insert(current.0);

        let current_pos = current.0;

        for neighbor in [
            (current_pos.0 + 1, current_pos.1),
            (current_pos.0 - 1, current_pos.1),
            (current_pos.0, current_pos.1 + 1),
            (current_pos.0, current_pos.1 - 1),
        ].iter() {
            let distance = current.1 + 1;

            if *neighbor == target {
                return Some(distance);
            } else if !examined.contains(neighbor) && {
                let field = map.get(&neighbor).unwrap_or(&Field::Undefined);
                *field == Field::Empty
            } {
                open.insert((*neighbor, distance));
            }
        }
    }
}

