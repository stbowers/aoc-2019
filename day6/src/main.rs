use aoc_utils::prelude::*;
use std::env;
use std::time::Instant;

fn count_orbits(body: &str, rel_to: &str, map: &HashMap<&str, &str>) -> usize {
    match body {
        rel_to => return 0,
        body => return 1 + count_orbits(map.get(body).unwrap(), rel_to, map),
    }
}

// Is body1 an ancestor of body2 (i.e. does body2 orbit body1 directly or indirectly)
fn is_ancestor(body1: &str, body2: &str, map: &HashMap<&str, &str>) -> bool {
    let body2_parent = map.get(body2).unwrap();
    if &body1 == body2_parent {
        return true;
    } else if body2_parent == &"COM" {
        return false;
    } else {
        return is_ancestor(body1, body2_parent, map);
    }
}

fn get_common_ancestor<'a>(body1: &'a str, body2: &'a str, map: &HashMap<&'a str, &'a str>) -> &'a str {
    if is_ancestor(body1, body2, map) {
        return body1;
    } else if is_ancestor(body2, body1, map) {
        return body2;
    } else {
        return get_common_ancestor(map.get(body1).unwrap(), body2, map);
    }
}

fn get_num_transfers(map: &HashMap<&str, &str>, obj1: &str, obj2: &str) -> usize {
    if obj1 == obj2 {
        return 0;
    } else if is_ancestor(obj1, obj2, map) {
        return get_num_transfers(map, obj1, map.get(obj2).unwrap()) + 1;
    } else if is_ancestor(obj2, obj1, map) {
        return get_num_transfers(map, obj2, map.get(obj1).unwrap()) + 1;
    } else {
        let common_ancestor = get_common_ancestor(obj1, obj2, map);
        return get_num_transfers(map, obj1, common_ancestor) + get_num_transfers(map, obj2, common_ancestor);
    }
}

fn part1(input: &str) {
    let mut input_buf = String::new();
    let mut orbits: HashMap<&str, &str> = HashMap::new();
    read_lines(input, &mut input_buf, true).unwrap().iter()
    	.for_each(|line| {
            let parts: Vec<&str> = line.split(")").collect();
            let parent = parts[0];
            let id = parts[1];
            orbits.insert(id, parent);
    	});

    let mut total_orbits = 0;
    for orbit in orbits.keys() {
        total_orbits += count_orbits(orbit, "COM", &orbits);
    }

    println!("total orbits: {}", total_orbits);
}

fn part2(input: &str) {
    let mut input_buf = String::new();
    let mut orbits: HashMap<&str, &str> = HashMap::new();
    read_lines(input, &mut input_buf, true).unwrap().iter()
    	.for_each(|line| {
            let parts: Vec<&str> = line.split(")").collect();
            let parent = parts[0];
            let id = parts[1];
            orbits.insert(id, parent);
    	});

    println!("num transfers: {}", get_num_transfers(&orbits, "YOU", "SAN") - 2);
}

fn main() {
    let mut test = false;
    env::args().for_each(|opt| match &opt as &str {
        "--test" => { test = true },
        _ => (),
    });

    let input = if test {
        "./test_input.txt"
    } else {
        "./input.txt"
    };

    println!("Advent of Code, day 1");
    let mut now: Instant;

    println!("===== Part 1 =====");
    now = Instant::now();
    part1(input);
    println!("=> {}s", now.elapsed().as_secs_f32());

    println!("===== Part 2 =====");
    now = Instant::now();
    part2(input);
    println!("=> {}s", now.elapsed().as_secs_f32());
}

