use aoc_utils::prelude::*;
use std::env;
use std::time::Instant;

fn calc_fuel(mass: i32) -> i32 {
    return (mass / 3) - 2;
}

fn part1(input: &str) {
    let mut input_buf = String::new();
    let total_fuel: i32 = read_lines(input, &mut input_buf, true).unwrap()
        .iter()
        .map(|line| {
            let mass: i32;
            scan!(line.bytes() => "{}", mass);
            return calc_fuel(mass);
        })
        .sum();

    println!("Total fuel: {}", total_fuel);
}

fn part2(input: &str) {
    let mut input_buf = String::new();
    let total_fuel: i32 = read_lines(input, &mut input_buf, true).unwrap()
        .iter()
        .map(|line| {
            let mass: i32;
            scan!(line.bytes() => "{}", mass);

            let mut fuel = calc_fuel(mass);
            let mut extra_fuel = calc_fuel(fuel);
            while extra_fuel > 0 {
                fuel += extra_fuel;
                extra_fuel = calc_fuel(extra_fuel);
            }

            return fuel;
        })
        .sum();

    println!("Total fuel: {}", total_fuel);
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

