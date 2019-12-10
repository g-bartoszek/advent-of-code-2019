use std::collections::HashSet;


type Position = (i32,i32);

fn distance(l: &Position, r: &Position) -> f32 {
    (((l.0 - r.0).pow(2) + (l.1 - r.1).pow(2)) as f32).sqrt()
}

fn is_visible(asteroids: &HashSet<Position>, target: &Position, origin: &Position) -> bool {

    let x = target.0 - origin.0;
    let y = target.1 - origin.1;

    for c in asteroids {
        if c == target {
            continue;
        }

        let cx = c.0 - origin.0;
        let cy = c.1 - origin.1;

        let to_target = distance(origin, target);
        let to_checked = distance(origin, c);


        if ((x as f32 / to_target - cx as f32 / to_checked).abs() < 0.00001) &&
            ((y as f32 / to_target - cy as f32 / to_checked).abs() < 0.00001) &&
                distance(origin, c) < (distance(origin, target)){
            //println!("{:?} can't see {:?} because of {:?}", origin, target, c);
            return false;
        }


    }


    true
}

fn main() {
    println!("Hello, world!");
    let test_data1 = String::from(
   ".#..#\n\
    .....\n\
    #####\n\
    ....#\n\
    ...##"
    );

    let test_data2 = String::from(
   ".###..#######..####..##...#\n\
    ########.#.###...###.#....#\n\
    ###..#...#######...#..####.\n\
    .##.#.....#....##.#.#.....#\n\
    ###.#######.###..##......#.\n\
    #..###..###.##.#.#####....#\n\
    #.##..###....#####...##.##.\n\
    ####.##..#...#####.#..###.#\n\
    #..#....####.####.###.#.###\n\
    #..#..#....###...#####..#..\n\
    ##...####.######....#.####.\n\
    ####.##...###.####..##....#\n\
    #.#..#.###.#.##.####..#...#\n\
    ..##..##....#.#..##..#.#..#\n\
    ##.##.#..######.#..#..####.\n\
    #.....#####.##........#####\n\
    ###.#.#######..#.#.##..#..#\n\
    ###...#..#.#..##.##..#####.\n\
    .##.#..#...#####.###.##.##.\n\
    ...#.#.######.#####.#.####.\n\
    #..##..###...###.#.#..#.#.#\n\
    .#..#.#......#.###...###..#\n\
    #.##.#.#..#.#......#..#..##\n\
    .##.##.##.#...##.##.##.#..#\n\
    #.###.#.#...##..#####.###.#\n\
    #.####.#..#.#.##.######.#..\n\
    .#.#####.##...#...#.##...#.");
    let test_data3 = String::from(
   ".#..##.###...#######\n\
    ##.############..##.\n\
    .#.######.########.#\n\
    .###.#######.####.#.\n\
    #####.##.#.##.###.##\n\
    ..#####..#.#########\n\
    ####################\n\
    #.####....###.#.#.##\n\
    ##.#################\n\
    #####.##.###..####..\n\
    ..######..##.#######\n\
    ####.##.####...##..#\n\
    .#####..#.######.###\n\
    ##...#.##########...\n\
    #.##########.#######\n\
    .####.#.###.###.#.##\n\
    ....##.##.###..#####\n\
    .#.#.###########.###\n\
    #.#.#.#####.####.###\n\
    ###.##.####.##.#..##");

    let test_data4 = String::from(
   "......#.#.\n\
    #..#.#....\n\
    ..#######.\n\
    .#.#.###..\n\
    .#..#.....\n\
    ..#....#.#\n\
    #..#....#.\n\
    .##.#..###\n\
    ##...#..#.\n\
    .#....####");

    let test_data5 = String::from(
   ".#..#..###\n\
    ####.###.#\n\
    ....###.#.\n\
    ..###.##.#\n\
    ##.##.#.#.\n\
    ....###..#\n\
    ..#.#..#.#\n\
    #..#.#.###\n\
    .##...##.#\n\
    .....#.#..");

    let test_data = test_data2;

    let HEIGHT = test_data.lines().count();
    let WIDTH = test_data.lines().next().unwrap().chars().count();

    println!("{} {}", WIDTH, HEIGHT);


    let asteroids  = test_data.lines().enumerate().fold(std::collections::HashSet::<(i32, i32)>::new(), |mut acc, (y, l)| {
        l.chars().enumerate().for_each(|(x,c)| {
            if c == '#' {
                acc.insert((x as i32,y as i32));
            }

        });
        acc
    });

    println!("{:?}", asteroids);

    let (mut max, mut pos) = (0, (0,0));

    for o in &asteroids {
        let mut counter = 0;
        for t in &asteroids {
           if o != t && is_visible(&asteroids, &t, &o) {
               counter += 1;
           }
        }

        //println!("{:?} {}", o, counter);
        if counter > max {
            max = counter;
            pos = *o;
        }
    }

    println!("MAX: {} {:?}", max, pos);


    let mut other = asteroids.iter().filter(|&a| *a != pos).map(|a| (a, angle(&(0.0f32, 1.0f32), ((a.0 - pos.0) as f32, (a.1 - pos.1) as f32)))).collect::<Vec<_>>();
    other.sort_by(|l, r| l.1.partial_cmp(&r.1).unwrap());

    println!("{:?}", other);

    let mut directions = Vec::<Vec<Position>>::new();
    let mut current = vec![*other[0].0];
    for w in other.windows(2) {
        if (w[0].1 - w[1].1).abs() > 0.000001 {
            current.sort_by(|l,r| distance(l, &pos).partial_cmp(&distance(r, &pos)).unwrap());
            directions.push(current.clone());
            current.clear();
        }
        current.push(*w[1].0);
    }
    current.sort_by(|l,r| distance(l, &pos).partial_cmp(&distance(r, &pos)).unwrap());
    directions.push(current.clone());

    println!("{:?}", directions);

    let mut cycle = 0usize;
    for c in 0..200 {
        loop {
            let l = directions.len();
            let d = &mut directions[cycle];
            cycle = (cycle + 1) % l;
            if !d.is_empty() {
                println!("{} Removed: {:?}", c, d.remove(0));
                break;
            }

        }
    }
}


fn angle(l: &(f32, f32), r: (f32, f32)) -> f32 {
    let dot = l.0*r.0 + l.1*r.1;
    let det = l.0*r.1 - l.1*r.0;
    det.atan2(dot)
}