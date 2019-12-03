use aoc_utils::prelude::*;
use std::env;
use std::time::Instant;

fn get_dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
}

fn part1(input: &str) {
    let mut input_buf = String::new();
    let lines: Vec<Vec<(i32, i32, i32, i32)>> = read_lines(input, &mut input_buf, true).unwrap()
        .iter()
        .map(|line| {
            let mut p = (0, 0);
            let mut lines = Vec::new();
            for direction in line.split(",") {
                let mut dx: i32 = 0;
                let mut dy: i32 = 0;

                if direction.starts_with("R") {
                    dx = direction[1..].parse::<i32>().unwrap();
                } else if direction.starts_with("L") {
                    dx = -direction[1..].parse::<i32>().unwrap();
                } else if direction.starts_with("U") {
                    dy = direction[1..].parse::<i32>().unwrap();
                } else if direction.starts_with("D") {
                    dy = -direction[1..].parse::<i32>().unwrap();
                }

                lines.push((p.0, p.1, p.0 + dx, p.1 + dy));
                p = (p.0 + dx, p.1 + dy);
            }
            lines
        })
        .collect();

    let wire1_lines = &lines[0];
    let wire2_lines = &lines[1];

    let mut closest = (i32::max_value(), i32::max_value());
    let mut closest_dist = i32::max_value();

    for wire1_line in wire1_lines {
        for wire2_line in wire2_lines {
            println!("Testing lines {:?} and {:?}", wire1_line, wire2_line);
            let (w1_x1, w1_y1, w1_x2, w1_y2) = wire1_line;
            let (w2_x1, w2_y1, w2_x2, w2_y2) = wire2_line;

            let w1_dx = w1_x2 - w1_x1;
            let w2_dx = w2_x2 - w2_x1;
            let w1_dy = w1_y2 - w1_y1;
            let w2_dy = w2_y2 - w2_y1;

            if (w1_dx == 0 && w2_dy == 0) {
                // w1 is vertical, w2 is horizontal
                let w2_low_x = i32::min(*w2_x1, *w2_x2);
                let w2_high_x = i32::max(*w2_x1, *w2_x2);
                if (*w1_x1 <= w2_high_x) && (*w1_x1 >= w2_low_x) {
                    let w1_low_y = i32::min(*w1_y1, *w1_y2);
                    let w1_high_y = i32::max(*w1_y1, *w1_y2);
                    if (*w2_y1 <= w1_high_y) && (*w2_y1 >= w1_low_y) {
                        let dist = get_dist((0, 0), (*w1_x1, *w2_y1));
                        println!("lines: {:?} and {:?} cross at {:?}, (dist: {})", wire1_line, wire2_line, (*w1_x1, *w2_y1), dist);
                        if dist < closest_dist && dist != 0{
                            closest = (*w1_x1, *w2_y1);
                            closest_dist = dist;
                        }
                    }
                }
            }

            if (w1_dy == 0 && w2_dx == 0) {
                // w2 is vertical, w1 is horizontal
                let w1_low_x = i32::min(*w1_x1, *w1_x2);
                let w1_high_x = i32::max(*w1_x1, *w1_x2);
                if (*w2_x1 <= w1_high_x) && (*w2_x1 >= w1_low_x) {
                    let w2_low_y = i32::min(*w2_y1, *w2_y2);
                    let w2_high_y = i32::max(*w2_y1, *w2_y2);
                    if (*w1_y1 <= w2_high_y) && (*w1_y1 >= w2_low_y) {
                        let dist = get_dist((0, 0), (*w2_x1, *w1_y1));
                        if dist < closest_dist && dist != 0{
                            println!("x lines: {:?} and {:?} cross at {:?}, (dist: {})", wire1_line, wire2_line, (*w2_x1, *w1_y1), dist);
                            closest = (*w2_x1, *w1_y1);
                            closest_dist = dist;
                        }
                    }
                }
            }

        }
    }

    println!("Closest distance: {} (at point {:?})", closest_dist, closest);
}

fn part2(input: &str) {
    let mut input_buf = String::new();
    let lines: Vec<Vec<(i32, i32, i32, i32, i32)>> = read_lines(input, &mut input_buf, true).unwrap()
        .iter()
        .map(|line| {
            let mut p = (0, 0);
            let mut steps = 0;
            let mut lines = Vec::new();
            for direction in line.split(",") {
                let mut dx: i32 = 0;
                let mut dy: i32 = 0;

                if direction.starts_with("R") {
                    dx = direction[1..].parse::<i32>().unwrap();
                } else if direction.starts_with("L") {
                    dx = -direction[1..].parse::<i32>().unwrap();
                } else if direction.starts_with("U") {
                    dy = direction[1..].parse::<i32>().unwrap();
                } else if direction.starts_with("D") {
                    dy = -direction[1..].parse::<i32>().unwrap();
                }

                steps = steps + dx.abs() + dy.abs();
                lines.push((p.0, p.1, p.0 + dx, p.1 + dy, steps));
                p = (p.0 + dx, p.1 + dy);
            }
            lines
        })
        .collect();

    let wire1_lines = &lines[0];
    let wire2_lines = &lines[1];

    let mut lowest_steps = i32::max_value();

    for wire1_line in wire1_lines {
        for wire2_line in wire2_lines {
            println!("Testing lines {:?} and {:?}", wire1_line, wire2_line);
            let (w1_x1, w1_y1, w1_x2, w1_y2, w1_steps) = wire1_line;
            let (w2_x1, w2_y1, w2_x2, w2_y2, w2_steps) = wire2_line;

            let w1_dx = w1_x2 - w1_x1;
            let w2_dx = w2_x2 - w2_x1;
            let w1_dy = w1_y2 - w1_y1;
            let w2_dy = w2_y2 - w2_y1;

            if (w1_dx == 0 && w2_dy == 0) {
                // w1 is vertical, w2 is horizontal
                let w2_low_x = i32::min(*w2_x1, *w2_x2);
                let w2_high_x = i32::max(*w2_x1, *w2_x2);
                if (*w1_x1 <= w2_high_x) && (*w1_x1 >= w2_low_x) {
                    let w1_low_y = i32::min(*w1_y1, *w1_y2);
                    let w1_high_y = i32::max(*w1_y1, *w1_y2);
                    if (*w2_y1 <= w1_high_y) && (*w2_y1 >= w1_low_y) {
                        let dist = get_dist((0, 0), (*w1_x1, *w2_y1));
                        let steps = w1_steps + w2_steps - ((*w1_y1 - *w1_y2).abs() - (*w1_y1 - *w2_y1).abs()) - ((*w2_x1 - *w2_x2).abs() - (*w2_x1 - *w1_x1).abs());
                            println!("lines: {:?} and {:?} cross at {:?}, (steps: {})", wire1_line, wire2_line, (*w1_x1, *w2_y1), steps);
                        if steps < lowest_steps && dist != 0{
                            lowest_steps = steps;
                        }
                    }
                }
            }

            if (w1_dy == 0 && w2_dx == 0) {
                // w2 is vertical, w1 is horizontal
                let w1_low_x = i32::min(*w1_x1, *w1_x2);
                let w1_high_x = i32::max(*w1_x1, *w1_x2);
                if (*w2_x1 <= w1_high_x) && (*w2_x1 >= w1_low_x) {
                    let w2_low_y = i32::min(*w2_y1, *w2_y2);
                    let w2_high_y = i32::max(*w2_y1, *w2_y2);
                    if (*w1_y1 <= w2_high_y) && (*w1_y1 >= w2_low_y) {
                        let dist = get_dist((0, 0), (*w2_x1, *w1_y1));
                        let steps = w1_steps + w2_steps - ((*w2_y1 - *w2_y2).abs() - (*w2_y1 - *w1_y1).abs()) - ((*w1_x1 - *w1_x2).abs() - (*w1_x1 - *w2_x1).abs());
                            println!("x lines: {:?} and {:?} cross at {:?}, (steps: {})", wire1_line, wire2_line, (*w2_x1, *w1_y1), steps);
                        if steps < lowest_steps && dist != 0{
                            lowest_steps = steps;
                        }
                    }
                }
            }

        }
    }

    println!("lowest steps: {}", lowest_steps);
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

