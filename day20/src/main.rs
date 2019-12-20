use std::io::BufRead;
use std::cmp::Ordering;

#[derive(PartialEq, Debug, Clone)]
enum Field {
    Wall,
    Empty,
    Space,
    Undefined,
}

type Point = (i32,i32);
type Map = std::collections::HashMap<Point, Field>;

fn main() {
    let input = std::io::BufReader::new(std::fs::File::open("input").unwrap());
    let mut map = Map::new();
    let mut keys = Vec::<(char, Point)>::new();
    let mut letters = std::collections::HashMap::<Point, char>::new();
    let mut portals = std::collections::HashMap::<[char;2], Point>::new();
    let mut start = (0,0);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.unwrap().chars().enumerate() {
            print!("{}", c);
            let x = x as i32;
            let y = y as i32;
            map.insert((x,y), match c {
                '#' => Field::Wall,
                '.' => Field::Empty,
                ' ' => Field::Space,
                _ => {
                    letters.insert((x,y), c);
                    Field::Space
                }
            });
        }
        println!();
    }

    for (position, field) in &map {
        if *field == Field::Empty {
            if *map.get(&(position.0 - 1, position.1)).unwrap_or(&Field::Undefined) == Field::Space {
                let mut name = [letters[&(position.0 - 1, position.1)], letters[&(position.0 - 2, position.1)]];
                if portals.contains_key(&name){
                    name.reverse();
                }

                portals.insert(name, *position);
            }

            if *map.get(&(position.0 + 1, position.1)).unwrap_or(&Field::Undefined) == Field::Space {
                let mut name = [letters[&(position.0 + 1, position.1)], letters[&(position.0 + 2, position.1)]];
                if portals.contains_key(&name){
                    name.reverse();
                }
                portals.insert(name, *position);
            }

            if *map.get(&(position.0, position.1 - 1)).unwrap_or(&Field::Undefined) == Field::Space {
                let mut name = [letters[&(position.0, position.1 - 1)], letters[&(position.0, position.1 - 2)]];
                if portals.contains_key(&name){
                    name.reverse();
                }
                portals.insert(name, *position);
            }

            if *map.get(&(position.0, position.1 + 1)).unwrap_or(&Field::Undefined) == Field::Space {
                let mut name = [letters[&(position.0, position.1 + 1)], letters[&(position.0, position.1 + 2)]];
                if portals.contains_key(&name){
                    name.reverse();
                }
                portals.insert(name, *position);
            }
        }
    }

    println!("{:?}", portals);
    let mut paths = std::collections::HashMap::<Point, Vec<(Point,usize)>>::new();

    for from in &portals {
        for to in &portals {
            if *from.0 == ['Z','Z'] || from == to || *to.0 == ['A','A'] {
                continue;
            }
            if let Some(d) = find_path(&map, *from.1, *to.1) {
                println!("Path from {:?} to {:?} is {:?}", from, to, d);
                if *to.0 ==  ['Z','Z'] {
                    //println!("It's target");
                    paths.entry(*from.1).or_insert(Vec::<(Point, usize)>::new()).push((*to.1, d));
                } else {
                    let mut target_portal = *to.0;
                    target_portal.reverse();
                    let to = portals[&target_portal];
                    //println!("Teleport to {:?} {:?}", target_portal,  to);
                    paths.entry(*from.1).or_insert(Vec::<(Point, usize)>::new()).push((to,d+1));
                }
            }

        }
    }
    println!("{:?}", paths);

    /////////////SEARCH
    let mut examined = std::collections::HashSet::<Point>::new();
    let mut open = std::collections::HashSet::<(Point,usize)>::new();

    open.insert((portals[&['A','A']], 0));

    loop {
        if open.is_empty() {
            panic!("PATH NOT FOUND");
        }

        let current = *open.iter().min_by(|((lx,ly), l), ((rx,ry), r)| {
            l.cmp(&r)
        }).unwrap();
        open.remove(&current);

        examined.insert(current.0);

        let current_pos = current.0;
        if current_pos == portals[&['Z','Z']] {
            println!("DISTANCE: {}", current.1);
            return;
        }

        for neighbor in &paths[&current_pos] {
        let distance = current.1 + neighbor.1;

        if !examined.contains(&neighbor.0)  {
            open.insert((neighbor.0, distance));
        }
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


fn find_path(map: &Map, start: Point, target: Point, ) -> Option<usize> {
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

