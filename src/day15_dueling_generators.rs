pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input15.txt").unwrap();
    let (generator_a, generator_b) = parse(&input);
    println!(
        "after 40M pairs, count = {}",
        judge(generator_a, generator_b, 40_000_000)
    );
    let (generator_a, generator_b) = parse_v2(&input);
    println!(
        "v2: after 5M pairs, count = {}",
        judge(generator_a, generator_b, 5_000_000)
    );
}

fn parse(input: &str) -> (Generator, Generator) {
    let mut values = input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap());
    (
        Generator::new(16807, values.next().unwrap(), 1),
        Generator::new(48271, values.next().unwrap(), 1),
    )
}

fn parse_v2(input: &str) -> (Generator, Generator) {
    let mut values = input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap());
    (
        Generator::new(16807, values.next().unwrap(), 4),
        Generator::new(48271, values.next().unwrap(), 8),
    )
}

fn judge(mut generator_a: Generator, mut generator_b: Generator, rounds: u64) -> u64 {
    let mut score = 0;
    for _ in 0..rounds {
        generator_a.next();
        generator_b.next();
        if trailing_16_bits_equal(generator_a.value, generator_b.value) {
            score += 1;
        }
    }
    score
}

fn trailing_16_bits_equal(a: u64, b: u64) -> bool {
    const SIXTEEN_BITS: u64 = 0b111_111_111_111_111_1;
    (a & SIXTEEN_BITS) == (b & SIXTEEN_BITS)
}

struct Generator {
    factor: u64,
    value: u64,
    multiple: u64,
}

impl Generator {
    const MODULO: u64 = 2147483647;

    pub fn new(factor: u64, starting_value: u64, multiple: u64) -> Self {
        Self {
            factor,
            value: starting_value,
            multiple,
        }
    }

    pub fn next(&mut self) {
        loop {
            self.value = (self.value * self.factor) % Self::MODULO;
            if self.value % self.multiple == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (generator_a, generator_b) = parse("generator A uses 65\nwhile generator B uses 8921");

        assert_eq!(judge(generator_a, generator_b, 40_000_000), 588);
    }
    #[test]
    fn test2() {
        let (generator_a, generator_b) =
            parse_v2("generator A uses 65\nwhile generator B uses 8921");

        assert_eq!(judge(generator_a, generator_b, 5_000_000), 309);
    }
}
