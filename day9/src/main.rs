use aoc_utils::prelude::*;
use std::env;
use std::time::Instant;

use std::sync::mpsc;

fn part1(input: &str) {
    let mut input_buf = String::new();
    let mut code: Vec<i64> = read_lines(input, &mut input_buf, true).unwrap()
        .iter()
        .flat_map(|line| line.split(','))
        .map(|intcode| intcode.parse().unwrap())
        .collect();

    let mut vm = IntcodeVM::new(code);

    let (mut input, mut vm_in) = mpsc::channel();
    let (mut vm_out, mut output) = mpsc::channel();
    input.send(1);
    vm.run(&mut vm_in, &mut vm_out);
    std::mem::drop(vm_out);

    for out_val in output.iter() {
        println!("{}", out_val);
    }
}

fn part2(input: &str) {
    let mut input_buf = String::new();
    let mut code: Vec<i64> = read_lines(input, &mut input_buf, true).unwrap()
        .iter()
        .flat_map(|line| line.split(','))
        .map(|intcode| intcode.parse().unwrap())
        .collect();

    let mut vm = IntcodeVM::new(code);

    let (mut input, mut vm_in) = mpsc::channel();
    let (mut vm_out, mut output) = mpsc::channel();
    input.send(2);
    vm.run(&mut vm_in, &mut vm_out);
    std::mem::drop(vm_out);

    for out_val in output.iter() {
        println!("{}", out_val);
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

