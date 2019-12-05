use aoc_utils::prelude::*;
use std::env;
use std::time::Instant;

fn read_value(code: &Vec<i32>, a: i32, mode: u8) -> i32 {
    match mode {
        0 => return code[a as usize],
        1 => return a,
        _ => panic!("Illegal read parameter mode: {}", mode),
    }
}

fn write_value(code: &mut Vec<i32>, a: i32, value: i32, mode: u8) {
    match mode {
        0 => code[a as usize] = value,
        _ => panic!("Illegal write parameter mode: {}", mode),
    }
}

fn sim(mut code: Vec<i32>) -> i32 {
    #[inline(always)]
    fn add(code: &mut Vec<i32>, ip: usize, parameter_modes: &str) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>3}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];
        let b = code[ip + 2];
        let c = code[ip + 3];

        // Add *a + *b, store in *c
        // println!("*{} = *{} + *{} = {} + {} = {}", c, a, b, code[a], code[b], code[a] + code[b]);
        write_value(code, c, read_value(code, a, parameter_modes[2]) + read_value(code, b, parameter_modes[1]), parameter_modes[0]);

        return ip+4;
    }

    #[inline(always)]
    fn mul(code: &mut Vec<i32>, ip: usize, parameter_modes: &str) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>3}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();
        let a = code[ip + 1];
        let b = code[ip + 2];
        let c = code[ip + 3];

        // Multiply *a * *b, store in *c
        // println!("*{} = *{} * *{} = {} * {} = {}", c, a, b, code[a], code[b], code[a] * code[b]);
        write_value(code, c, read_value(code, a, parameter_modes[2]) * read_value(code, b, parameter_modes[1]), parameter_modes[0]);

        return ip+4;
    }

    fn input(code: &mut Vec<i32>, ip: usize, parameter_modes: &str) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>1}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];

        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input);
        let value = user_input.trim().parse().unwrap();

        write_value(code, a, value, parameter_modes[0]);

        return ip+2;
    }

    fn output(code: &mut Vec<i32>, ip: usize, parameter_modes: &str) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>1}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];

        println!("{}", read_value(code, a, parameter_modes[0]));

        return ip+2;
    }

    fn jump_if_true(code: &mut Vec<i32>, ip: usize, parameter_modes: &str) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>2}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];
        let b = code[ip + 2];

	if read_value(code, a, parameter_modes[1]) != 0 {
    	    return read_value(code, b, parameter_modes[0]) as usize;
	}

        return ip+3;
    }

    fn jump_if_false(code: &mut Vec<i32>, ip: usize, parameter_modes: &str) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>2}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];
        let b = code[ip + 2];

	if read_value(code, a, parameter_modes[1]) == 0 {
    	    return read_value(code, b, parameter_modes[0]) as usize;
	}

        return ip+3;
    }

    fn less_than(code: &mut Vec<i32>, ip: usize, parameter_modes: &str) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>3}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];
        let b = code[ip + 2];
        let c = code[ip + 3];

	if read_value(code, a, parameter_modes[2]) < read_value(code, b, parameter_modes[1]) {
    	    write_value(code, c, 1, parameter_modes[0]);
	} else {
    	    write_value(code, c, 0, parameter_modes[0]);
	}

        return ip+4;
    }

    fn equal(code: &mut Vec<i32>, ip: usize, parameter_modes: &str) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>3}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];
        let b = code[ip + 2];
        let c = code[ip + 3];

	if read_value(code, a, parameter_modes[2]) == read_value(code, b, parameter_modes[1]) {
    	    write_value(code, c, 1, parameter_modes[0]);
	} else {
    	    write_value(code, c, 0, parameter_modes[0]);
	}

        return ip+4;
    }

    let opcode_functions: [&Fn(&mut Vec<i32>, usize, &str) -> usize; 8] = [
    	&add, &mul, &input, &output, &jump_if_true, &jump_if_false, &less_than, &equal
    ];

    let mut ip = 0;
    loop {
        let opcode_str = format!("{:0>2}", code[ip].to_string());
        let opcode: i32 = opcode_str[opcode_str.len()-2..].parse().unwrap();
        let parameter_modes = &opcode_str[..opcode_str.len()-2];
        let opcode_index = (opcode - 1) as usize;
        if opcode_index < opcode_functions.len() {
            ip = opcode_functions[opcode_index](&mut code, ip, parameter_modes);
        } else {
            match opcode {
                99 => break,
                _ => {
                    println!("Invalid opcode: {}", opcode);
                    break;
                },
            }
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

    sim(code);
}

fn part2(input: &str) {
    let mut input_buf = String::new();
    let mut code: Vec<i32> = read_lines(input, &mut input_buf, true).unwrap()
        .iter()
        .flat_map(|line| line.split(','))
        .map(|intcode| intcode.parse().unwrap())
        .collect();

    sim(code);
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

