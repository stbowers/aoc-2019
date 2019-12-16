
type OperationFn = Fn(&mut IntcodeVM);

struct IntcodeVM {
    ip: usize,
    memory: Vec<i32>,
}

impl IntcodeVM {
    pub fn new(code: Vec<i32>) -> IntcodeVM {
    };

    fn add(&mut self) {
        let a = self.memory[self.ip + 1];
        let b = self.memory[self.ip + 1];
        let c = self.memory[self.ip + 1];

        self.ip += 4;
    }
}
