use aoc_utils::prelude::*;
use std::env;
use std::time::Instant;

fn part1(input: &str) {
    let low = 359282;
    let high = 820401;

    let num_paswords = (low..high+1).filter(|num| {
            let numstr = num.to_string();
            let has_pair = numstr.chars().collect::<Vec<char>>().windows(2).any(|win| win[0] == win[1]);
            let ordered = numstr.chars().collect::<Vec<char>>().windows(2).all(|win| win[0] <= win[1]);
            return has_pair && ordered;
        }).count();

    println!("answer: {}", num_paswords);
}

fn part2(input: &str) {
    let low = 359282;
    let high = 820401;

    let num_paswords = (low..high+1).filter(|num| {
            let numstr = num.to_string();
            let chars = numstr.chars().collect::<Vec<char>>();
            let has_pair = ((chars[0] == chars[1]) && chars[1] != chars[2])
                        || ((chars[1] == chars[2]) && chars[2] != chars[3] && chars[0] != chars[1])
                        || ((chars[2] == chars[3]) && chars[3] != chars[4] && chars[1] != chars[2])
                        || ((chars[3] == chars[4]) && chars[4] != chars[5] && chars[2] != chars[3])
                        || ((chars[4] == chars[5]) && chars[3] != chars[4]);
            let ordered = numstr.chars().collect::<Vec<char>>().windows(2).all(|win| win[0] <= win[1]);
            return has_pair && ordered;
        }).count();

    println!("answer: {}", num_paswords);
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

