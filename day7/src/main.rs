use aoc_utils::prelude::*;
use std::env;
use std::time::Instant;

use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

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

fn sim(mut code: Vec<i32>, data_input: &mut Vec<i32>, data_output: &mut Vec<i32>) -> i32 {
    #[inline(always)]
    fn add(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Vec<i32>, data_output: &mut Vec<i32>) -> usize {
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
    fn mul(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Vec<i32>, data_output: &mut Vec<i32>) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>3}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();
        let a = code[ip + 1];
        let b = code[ip + 2];
        let c = code[ip + 3];

        // Multiply *a * *b, store in *c
        // println!("*{} = *{} * *{} = {} * {} = {}", c, a, b, code[a], code[b], code[a] * code[b]);
        write_value(code, c, read_value(code, a, parameter_modes[2]) * read_value(code, b, parameter_modes[1]), parameter_modes[0]);

        return ip+4;
    }

    fn input(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Vec<i32>, data_output: &mut Vec<i32>) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>1}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];

        // let mut user_input = String::new();
        // std::io::stdin().read_line(&mut user_input);
        // let value = user_input.trim().parse().unwrap();
        let value = data_input.remove(0);

        write_value(code, a, value, parameter_modes[0]);

        return ip+2;
    }

    fn output(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Vec<i32>, data_output: &mut Vec<i32>) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>1}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];

        // println!("{}", read_value(code, a, parameter_modes[0]));
        data_output.push(read_value(code, a, parameter_modes[0]));

        return ip+2;
    }

    fn jump_if_true(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Vec<i32>, data_output: &mut Vec<i32>) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>2}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];
        let b = code[ip + 2];

	if read_value(code, a, parameter_modes[1]) != 0 {
    	    return read_value(code, b, parameter_modes[0]) as usize;
	}

        return ip+3;
    }

    fn jump_if_false(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Vec<i32>, data_output: &mut Vec<i32>) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>2}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];
        let b = code[ip + 2];

    	if read_value(code, a, parameter_modes[1]) == 0 {
    	    return read_value(code, b, parameter_modes[0]) as usize;
    	}

        return ip+3;
    }

    fn less_than(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Vec<i32>, data_output: &mut Vec<i32>) -> usize {
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

    fn equal(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Vec<i32>, data_output: &mut Vec<i32>) -> usize {
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

    let opcode_functions: [&Fn(&mut Vec<i32>, usize, &str, &mut Vec<i32>, &mut Vec<i32>) -> usize; 8] = [
    	&add, &mul, &input, &output, &jump_if_true, &jump_if_false, &less_than, &equal
    ];

    let mut ip = 0;
    loop {
        let opcode_str = format!("{:0>2}", code[ip].to_string());
        let opcode: i32 = opcode_str[opcode_str.len()-2..].parse().unwrap();
        let parameter_modes = &opcode_str[..opcode_str.len()-2];
        let opcode_index = (opcode - 1) as usize;
        if opcode_index < opcode_functions.len() {
            ip = opcode_functions[opcode_index](&mut code, ip, parameter_modes, data_input, data_output);
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

fn sim2(mut code: Vec<i32>, mut data_input: Receiver<i32>, mut data_output: Sender<i32>) -> Receiver<i32> {
    #[inline(always)]
    fn add(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i32>, data_output: &mut Sender<i32>) -> usize {
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
    fn mul(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i32>, data_output: &mut Sender<i32>) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>3}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();
        let a = code[ip + 1];
        let b = code[ip + 2];
        let c = code[ip + 3];

        // Multiply *a * *b, store in *c
        // println!("*{} = *{} * *{} = {} * {} = {}", c, a, b, code[a], code[b], code[a] * code[b]);
        write_value(code, c, read_value(code, a, parameter_modes[2]) * read_value(code, b, parameter_modes[1]), parameter_modes[0]);

        return ip+4;
    }

    fn input(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i32>, data_output: &mut Sender<i32>) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>1}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];

        // let mut user_input = String::new();
        // std::io::stdin().read_line(&mut user_input);
        // let value = user_input.trim().parse().unwrap();

        // let value = data_input.remove(0);

        let value = data_input.recv().unwrap();

        write_value(code, a, value, parameter_modes[0]);

        return ip+2;
    }

    fn output(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i32>, data_output: &mut Sender<i32>) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>1}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];

        // println!("{}", read_value(code, a, parameter_modes[0]));
        // data_output.push(read_value(code, a, parameter_modes[0]));
        data_output.send(read_value(code, a, parameter_modes[0])).unwrap();

        return ip+2;
    }

    fn jump_if_true(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i32>, data_output: &mut Sender<i32>) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>2}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];
        let b = code[ip + 2];

	if read_value(code, a, parameter_modes[1]) != 0 {
    	    return read_value(code, b, parameter_modes[0]) as usize;
	}

        return ip+3;
    }

    fn jump_if_false(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i32>, data_output: &mut Sender<i32>) -> usize {
        let parameter_modes: Vec<u8> = format!("{:0>2}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        let a = code[ip + 1];
        let b = code[ip + 2];

    	if read_value(code, a, parameter_modes[1]) == 0 {
    	    return read_value(code, b, parameter_modes[0]) as usize;
    	}

        return ip+3;
    }

    fn less_than(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i32>, data_output: &mut Sender<i32>) -> usize {
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

    fn equal(code: &mut Vec<i32>, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i32>, data_output: &mut Sender<i32>) -> usize {
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

    let opcode_functions: [&Fn(&mut Vec<i32>, usize, &str, &mut Receiver<i32>, &mut Sender<i32>) -> usize; 8] = [
    	&add, &mul, &input, &output, &jump_if_true, &jump_if_false, &less_than, &equal
    ];

    let mut ip = 0;
    loop {
        let opcode_str = format!("{:0>2}", code[ip].to_string());
        let opcode: i32 = opcode_str[opcode_str.len()-2..].parse().unwrap();
        let parameter_modes = &opcode_str[..opcode_str.len()-2];
        let opcode_index = (opcode - 1) as usize;
        if opcode_index < opcode_functions.len() {
            ip = opcode_functions[opcode_index](&mut code, ip, parameter_modes, &mut data_input, &mut data_output);
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

    return data_input;
}

fn amplify(code: Vec<i32>, a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let mut input = vec![a, 0];
    let mut output = vec![];

    sim(code.clone(), &mut input, &mut output);
    input.push(b);
    input.push(output.pop().unwrap());
    sim(code.clone(), &mut input, &mut output);
    input.push(c);
    input.push(output.pop().unwrap());
    sim(code.clone(), &mut input, &mut output);
    input.push(d);
    input.push(output.pop().unwrap());
    sim(code.clone(), &mut input, &mut output);
    input.push(e);
    input.push(output.pop().unwrap());
    sim(code.clone(), &mut input, &mut output);

    return output[0];
}

fn amplify2(code: Vec<i32>, a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {

    let (e_out, a_in) = mpsc::channel();
    let (a_out, b_in) = mpsc::channel();
    let (b_out, c_in) = mpsc::channel();
    let (c_out, d_in) = mpsc::channel();
    let (d_out, e_in) = mpsc::channel();

    let a_code = code.clone();
    let b_code = code.clone();
    let c_code = code.clone();
    let d_code = code.clone();
    let e_code = code.clone();

    e_out.send(a);
    e_out.send(0);
    a_out.send(b);
    b_out.send(c);
    c_out.send(d);
    d_out.send(e);


    let amp_a = thread::spawn(move || {
        return sim2(a_code, a_in, a_out);
    });
    let amp_b = thread::spawn(move || {
        return sim2(b_code, b_in, b_out);
    });
    let amp_c = thread::spawn(move || {
        return sim2(c_code, c_in, c_out);
    });
    let amp_d = thread::spawn(move || {
        return sim2(d_code, d_in, d_out);
    });
    let amp_e = thread::spawn(move || {
        return sim2(e_code, e_in, e_out);
    });

    let a_in = amp_a.join().unwrap();
    amp_b.join().unwrap();
    amp_c.join().unwrap();
    amp_d.join().unwrap();
    amp_e.join().unwrap();

    return a_in.recv().unwrap();
}

fn part1(input: &str) {
    let mut input_buf = String::new();
    let mut code: Vec<i32> = read_lines(input, &mut input_buf, true).unwrap()
        .iter()
        .flat_map(|line| line.split(','))
        .map(|intcode| intcode.parse().unwrap())
        .collect();

    let mut max_output = 0;

    for a in 0..5 {
        for b in 0..5 {
            if b == a {continue;}
            for c in 0..5 {
                if c == a || c == b {continue;}
                for d in 0..5 {
                    if d == c || d == b || d == a {continue;}
                    for e in 0..5 {
                        if e == d || e == c || e == b || e == a {continue;}
                        let output = amplify(code.clone(), a, b, c, d, e);
                        if output > max_output {
                            println!("{:?} => {}", (a, b, c, d, e), output);
                            max_output = output;
                        }
                    }
                }
            }
        }
    }

    println!("max output: {}", max_output);
}

fn part2(input: &str) {
    let mut input_buf = String::new();
    let mut code: Vec<i32> = read_lines(input, &mut input_buf, true).unwrap()
        .iter()
        .flat_map(|line| line.split(','))
        .map(|intcode| intcode.parse().unwrap())
        .collect();

    let mut max_output = 0;

    for a in 5..10 {
        for b in 5..10 {
            if b == a {continue;}
            for c in 5..10 {
                if c == a || c == b {continue;}
                for d in 5..10 {
                    if d == c || d == b || d == a {continue;}
                    for e in 5..10 {
                        if e == d || e == c || e == b || e == a {continue;}
                        let output = amplify2(code.clone(), a, b, c, d, e);
                        if output > max_output {
                            println!("{:?} => {}", (a, b, c, d, e), output);
                            max_output = output;
                        }
                    }
                }
            }
        }
    }

    println!("max output: {}", max_output);
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

