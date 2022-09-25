pub(crate) fn run() {
    let input = 328;
    let mut buffer = CircularBuffer::new(input);
    for _ in 0..2017 {
        buffer.next();
    }
    println!("short-circuit value: {}", buffer.short_circuit_value());

    // let mut buffer = FakeCircularBuffer::new(3);
    // for i in 1..=2017 {
    //     buffer.next();
    //     println!("after {}, value after 0: {}", i, buffer.value_after_zero);
    // }

    let mut buffer = FakeCircularBuffer::new(input);
    for _ in 0..50_000_000 {
        buffer.next();
    }
    println!(
        "after 50M values, value after 0: {}",
        buffer.value_after_zero
    );
}

struct CircularBuffer {
    step: usize,
    values: Vec<u32>,
    current_position: usize,
    last_value: u32,
}

impl CircularBuffer {
    pub fn new(step: usize) -> Self {
        Self {
            step,
            values: vec![0],
            current_position: 0,
            last_value: 0,
        }
    }
    pub fn next(&mut self) {
        self.last_value += 1;
        self.current_position = 1 + (self.current_position + self.step) % self.values.len();
        self.values.insert(self.current_position, self.last_value);
    }
    pub fn short_circuit_value(&self) -> u32 {
        self.values[(self.current_position + 1) % self.values.len()]
    }
}

struct FakeCircularBuffer {
    step: usize,
    current_position: usize,
    last_value: usize,
    zero_index: usize,
    value_after_zero: usize,
}

impl FakeCircularBuffer {
    pub fn new(step: usize) -> Self {
        Self {
            step,
            current_position: 0,
            last_value: 0,
            zero_index: 0,
            value_after_zero: 0,
        }
    }
    pub fn next(&mut self) {
        self.last_value += 1;
        let j = (self.current_position + self.step) % self.last_value;
        self.current_position = 1 + j;
        if self.zero_index == j {
            self.value_after_zero = self.last_value;
        } else if self.zero_index == self.current_position {
            self.zero_index = (self.zero_index + 1) % (self.last_value + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = 3;
        let mut buffer = CircularBuffer::new(input);
        for _ in 0..2017 {
            buffer.next();
        }
        assert_eq!(buffer.short_circuit_value(), 638)
    }
}
