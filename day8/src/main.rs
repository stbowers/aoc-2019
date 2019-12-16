use aoc_utils::prelude::*;
use std::env;
use std::time::Instant;

const IMG_WIDTH: usize = 25;
const IMG_HEIGHT: usize = 6;

fn part1(input: &str) {
    let mut input_buf = String::new();
    let raw_pixels: Vec<u8> = read_lines(input, &mut input_buf, true).unwrap().iter()
    	.flat_map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8))
    	.collect();

    let ppl = IMG_HEIGHT * IMG_WIDTH; // Pixels per layer
    let num_layers = raw_pixels.len() / ppl;

    let mut lowest_layer_index = 0;
    let mut lowest_layer_num_zeroes = usize::max_value();

    for layer_index in 0..num_layers {
        let num_zeroes = raw_pixels[layer_index*ppl..(layer_index+1)*ppl].iter().filter(|pixel| **pixel == 0).count();
        if num_zeroes < lowest_layer_num_zeroes {
            lowest_layer_index = layer_index;
            lowest_layer_num_zeroes = num_zeroes;
        }
    }

    // count number of 1s
    let num_ones = raw_pixels[lowest_layer_index*ppl..(lowest_layer_index+1)*ppl].iter().filter(|pixel| **pixel == 1).count();
    let num_twos = raw_pixels[lowest_layer_index*ppl..(lowest_layer_index+1)*ppl].iter().filter(|pixel| **pixel == 2).count();

    println!("Layer with fewest 0s: {}", lowest_layer_index);
    println!("|ones| * |twos| = {}", num_ones * num_twos);
}

fn part2(input: &str) {
    let mut input_buf = String::new();
    let raw_pixels: Vec<u8> = read_lines(input, &mut input_buf, true).unwrap().iter()
    	.flat_map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8))
    	.collect();

    let ppl = IMG_HEIGHT * IMG_WIDTH; // Pixels per layer
    let num_layers = raw_pixels.len() / ppl;

    let mut image: Vec<Vec<u8>> = vec![vec![2; IMG_HEIGHT]; IMG_WIDTH];

    for layer_index in 0..num_layers {
        for x in 0..IMG_WIDTH {
            for y in 0..IMG_HEIGHT {
                if image[x][y] == 2 {
                    image[x][y] = raw_pixels[(layer_index*ppl) + (y*IMG_WIDTH + x)];
                }
            }
        }
    }

    for y in 0..IMG_HEIGHT {
        for x in 0..IMG_WIDTH {
            match image[x][y] {
                0 => print!("."),
                1 => print!("#"),
                2 => print!(" "),
                _ => panic!("bad pixel at {}, {}", x, y),
            }
        }
        println!();
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

