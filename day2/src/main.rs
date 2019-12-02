use aoc_utils::prelude::*;
use std::env;
use std::time::Instant;

fn sim(mut code: Vec<i32>, noun: i32, verb: i32) -> i32 {
    code[1] = noun;
    code[2] = verb;

    #[inline(always)]
    fn add(code: &mut Vec<i32>, ip: usize) -> usize {
        let a = code[ip + 1] as usize;
        let b = code[ip + 2] as usize;
        let c = code[ip + 3] as usize;

        // Add *a + *b, store in *c
        // println!("*{} = *{} + *{} = {} + {} = {}", c, a, b, code[a], code[b], code[a] + code[b]);
        code[c] = code[a] + code[b];

        return 4;
    }

    #[inline(always)]
    fn mul(code: &mut Vec<i32>, ip: usize) -> usize {
        let a = code[ip + 1] as usize;
        let b = code[ip + 2] as usize;
        let c = code[ip + 3] as usize;

        // Multiply *a * *b, store in *c
        // println!("*{} = *{} * *{} = {} * {} = {}", c, a, b, code[a], code[b], code[a] * code[b]);
        code[c] = code[a] * code[b];

        return 4;
    }

    let mut ip = 0;
    loop {
        let intcode = code[ip];
        match intcode {
            1 => {
                ip += add(&mut code, ip);
            },
            2 => {
                ip += mul(&mut code, ip);
            },
            99 => {
                break;
            },
            _ => {
                println!("Unknown intcode: {}", intcode);
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

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let res = sim(code.clone(), noun, verb);
            if res == 19690720 {
                println!("Answer: {}", 100 * noun + verb);
                break 'outer;
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

