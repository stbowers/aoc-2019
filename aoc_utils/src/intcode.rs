use std::sync::mpsc::{Sender, Receiver};

pub struct IntcodeVM {
    ip: usize,
    code: Vec<i64>,
    rel_base: i64,
}

impl IntcodeVM {
    pub fn new(code: Vec<i64>) -> IntcodeVM {
        let mut vm = IntcodeVM {
            ip: 0,
            code: code,
            rel_base: 0,
        };

        vm.code.append(&mut vec![0; 1024]);

        return vm;
    }

    fn read_value(&self, a: i64, mode: u8) -> i64 {
        match mode {
            0 => return self.code[a as usize],                   // Position mode
            1 => return a,                                       // Immediate mode
            2 => return self.code[(a + self.rel_base) as usize], // Relative mode
            _ => panic!("Illegal read parameter mode: {}", mode),
        }
    }

    fn write_value(&mut self, a: i64, value: i64, mode: u8) {
        match mode {
            0 => self.code[a as usize] = value,
            1 => panic!("Cannot write in immediate mode..."),
            2 => self.code[(a + self.rel_base) as usize] = value,
            _ => panic!("Illegal write parameter mode: {}", mode),
        }
    }

    pub fn run(&mut self, data_input: &mut Receiver<i64>, data_output: &mut Sender<i64>) {
        #[inline(always)]
        fn add(this: &mut IntcodeVM, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i64>, data_output: &mut Sender<i64>) -> usize {
            let parameter_modes: Vec<u8> = format!("{:0>3}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

            let a = this.code[ip + 1];
            let b = this.code[ip + 2];
            let c = this.code[ip + 3];

            // Add *a + *b, store in *c
            // println!("*{} = *{} + *{} = {} + {} = {}", c, a, b, this.code[a], this.code[b], this.code[a] + this.code[b]);
            this.write_value(c, this.read_value(a, parameter_modes[2]) + this.read_value(b, parameter_modes[1]), parameter_modes[0]);

            return ip+4;
        }

        #[inline(always)]
        fn mul(this: &mut IntcodeVM, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i64>, data_output: &mut Sender<i64>) -> usize {
            let parameter_modes: Vec<u8> = format!("{:0>3}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();
            let a = this.code[ip + 1];
            let b = this.code[ip + 2];
            let c = this.code[ip + 3];

            // Multiply *a * *b, store in *c
            // println!("*{} = *{} * *{} = {} * {} = {}", c, a, b, this.code[a], this.code[b], this.code[a] * this.code[b]);
            this.write_value(c, this.read_value(a, parameter_modes[2]) * this.read_value(b, parameter_modes[1]), parameter_modes[0]);

            return ip+4;
        }

        fn input(this: &mut IntcodeVM, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i64>, data_output: &mut Sender<i64>) -> usize {
            let parameter_modes: Vec<u8> = format!("{:0>1}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

            let a = this.code[ip + 1];

            // let mut user_input = String::new();
            // std::io::stdin().read_line(&mut user_input);
            // let value = user_input.trim().parse().unwrap();

            // let value = data_input.remove(0);

            let value = data_input.recv().unwrap();

            this.write_value(a, value, parameter_modes[0]);

            return ip+2;
        }

        fn output(this: &mut IntcodeVM, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i64>, data_output: &mut Sender<i64>) -> usize {
            let parameter_modes: Vec<u8> = format!("{:0>1}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

            let a = this.code[ip + 1];

            // println!("{}", this.read_value(a, parameter_modes[0]));
            // data_output.push(this.read_value(a, parameter_modes[0]));
            data_output.send(this.read_value(a, parameter_modes[0])).unwrap();

            return ip+2;
        }

        fn jump_if_true(this: &mut IntcodeVM, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i64>, data_output: &mut Sender<i64>) -> usize {
            let parameter_modes: Vec<u8> = format!("{:0>2}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

            let a = this.code[ip + 1];
            let b = this.code[ip + 2];

    	if this.read_value(a, parameter_modes[1]) != 0 {
        	    return this.read_value(b, parameter_modes[0]) as usize;
    	}

            return ip+3;
        }

        fn jump_if_false(this: &mut IntcodeVM, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i64>, data_output: &mut Sender<i64>) -> usize {
            let parameter_modes: Vec<u8> = format!("{:0>2}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

            let a = this.code[ip + 1];
            let b = this.code[ip + 2];

        	if this.read_value(a, parameter_modes[1]) == 0 {
        	    return this.read_value(b, parameter_modes[0]) as usize;
        	}

            return ip+3;
        }

        fn less_than(this: &mut IntcodeVM, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i64>, data_output: &mut Sender<i64>) -> usize {
            let parameter_modes: Vec<u8> = format!("{:0>3}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

            let a = this.code[ip + 1];
            let b = this.code[ip + 2];
            let c = this.code[ip + 3];

        	if this.read_value(a, parameter_modes[2]) < this.read_value(b, parameter_modes[1]) {
        	    this.write_value(c, 1, parameter_modes[0]);
        	} else {
        	    this.write_value(c, 0, parameter_modes[0]);
        	}

            return ip+4;
        }

        fn equal(this: &mut IntcodeVM, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i64>, data_output: &mut Sender<i64>) -> usize {
            let parameter_modes: Vec<u8> = format!("{:0>3}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

            let a = this.code[ip + 1];
            let b = this.code[ip + 2];
            let c = this.code[ip + 3];

        	if this.read_value(a, parameter_modes[2]) == this.read_value(b, parameter_modes[1]) {
        	    this.write_value(c, 1, parameter_modes[0]);
        	} else {
        	    this.write_value(c, 0, parameter_modes[0]);
        	}

            return ip+4;
        }

        fn add_rel_base(this: &mut IntcodeVM, ip: usize, parameter_modes: &str, data_input: &mut Receiver<i64>, data_output: &mut Sender<i64>) -> usize {
            let parameter_modes: Vec<u8> = format!("{:0>1}", parameter_modes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();

            let a = this.code[ip + 1];

            // Adjust relative base
            this.rel_base += this.read_value(a, parameter_modes[0]);

            return ip+2;
        }

        let opcode_functions: [&Fn(&mut IntcodeVM, usize, &str, &mut Receiver<i64>, &mut Sender<i64>) -> usize; 9] = [
        	&add, &mul, &input, &output, &jump_if_true, &jump_if_false, &less_than, &equal, &add_rel_base
        ];

        loop {
            let opcode_str = format!("{:0>2}", self.code[self.ip].to_string());
            let opcode: i64 = opcode_str[opcode_str.len()-2..].parse().unwrap();
            let parameter_modes = &opcode_str[..opcode_str.len()-2];
            let opcode_index = (opcode - 1) as usize;
            if opcode_index < opcode_functions.len() {
                self.ip = opcode_functions[opcode_index](self, self.ip, parameter_modes, data_input, data_output);
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
    }
}

