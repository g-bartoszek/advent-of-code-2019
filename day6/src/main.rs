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

fn get_or_insert<K: std::cmp::Eq + std::hash::Hash, T:Default>(map: &mut std::collections::HashMap<K, T>, key: K) -> &mut T {
    map.entry(key).or_insert(T::default())
}

fn main() {

    let file = std::fs::File::open("input").unwrap();
    let input = std::io::BufReader::new(file);

    let mut data = Data::new();

    for l in input.lines() {
        let line = l.unwrap();
        let o = line.trim().split(')').collect::<Vec<_>>();
        get_or_insert(&mut data, o[0].to_string()).insert(o[1].to_string());
    }


    println!("Result: {}", data.keys().map(|k| count(&data, k)).sum::<usize>());

    let mut neighbors = data.clone();

    for (ref k, ref v) in data {
        for n in v {
            get_or_insert(&mut neighbors, n.clone()).insert(k.clone());
        }
    }

    find_route(&neighbors, None, &String::from("YOU"), 0);
}
