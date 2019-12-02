use aoc_utils::prelude::*;
use std::env;
use std::time::Instant;

fn sim(mut code: Vec<i32>, noun: i32, verb: i32) -> i32 {
    code[1] = noun;
    code[2] = verb;

    for i in 0..(code.len() / 4) {
        let ii = i * 4;
	match code[ii] {
    	    1 => {
       	        let x1 = code[ii + 1] as usize;
       	        let x2 = code[ii + 2] as usize;
       	        let x3 = code[ii + 3] as usize;
       	        code[x3] = code[x1] + code[x2];
    	    },
    	    2 => {
       	        let x1 = code[ii + 1] as usize;
       	        let x2 = code[ii + 2] as usize;
       	        let x3 = code[ii + 3] as usize;
       	        code[x3] = code[x1] * code[x2];
    	    },
            99 => { break; },
            _ => {
                println!("Unknown opcode: {}", code[ii]);
                break;
            },
	}
    }

    return code[0];
}

fn part1(input: &str) {
    let mut input_buf = String::new();
    let mut code: Vec<i32> = read_lines(input, &mut input_buf, true).unwrap()
        .iter()
        .flat_map(|line| line.split(','))
        .map(|intcode| intcode.parse().unwrap())
        .collect();

    println!("Answer: {}", sim(code, 12, 2));
}

fn part2(input: &str) {
    let mut input_buf = String::new();
    let mut code: Vec<i32> = read_lines(input, &mut input_buf, true).unwrap()
        .iter()
        .flat_map(|line| line.split(','))
        .map(|intcode| intcode.parse().unwrap())
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let res = sim(code.clone(), noun, verb);
            if res == 19690720 {
                println!("Answer: {}, {}", noun, verb);
            }
        }
    }
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

