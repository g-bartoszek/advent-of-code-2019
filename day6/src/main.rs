use std::collections::HashSet;
use std::io::BufRead;

type Data = std::collections::HashMap<String, std::collections::HashSet<String>>;

fn count(data: &Data, current: &String) -> usize {
    match data.get(current) {
        Some(satellites) => satellites.len() + satellites.iter().map(|s| count(data, s)).sum::<usize>(),
        None => 0
    }
}

fn find_route(data: &Data, from: Option<&String>, current: &String, current_length: usize) {
    for n in data.get(current).unwrap() {
        if let Some(f) = from {
            if f == n {
                continue;
            }
        }

        if n == "SAN" {
            println!("Found!: {}", current_length);
        } else {
            find_route(data, Some(current), n, current_length+1);
        }
    }

}

fn main() {

 /*   let input = "COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L
    K)YOU
    I)SAN";*/

    let file = std::fs::File::open("input").unwrap();
    let input = std::io::BufReader::new(file);

    let mut data = Data::new();

    for l in input.lines() {
        let line = l.unwrap();
        let o = line.trim().split(')').collect::<Vec<_>>();
        match data.get_mut(o[0]) {
            Some(entry) => {entry.insert(o[1].to_string()); },
            None => {
                let mut set = HashSet::new();
                set.insert(o[1].to_string());
                data.insert(o[0].to_string(), set);
            },
        }
    }

    //println!("Data {:?}", data);


    println!("Result: {}", data.keys().map(|k| count(&data, k)).sum::<usize>());

    let mut neighbors = data.clone();

    for (ref k, ref v) in data {
        for n in v {
            match neighbors.get_mut(n) {
                Some(entry) => {entry.insert(k.clone());},
                None => {
                    let mut set = HashSet::new();
                    set.insert(k.clone());
                    neighbors.insert(n.clone(), set);
                },
            }
        }
    }

    //println!("Neighbors {:?}", neighbors);

    find_route(&neighbors, None, &String::from("YOU"), 0);
}
