use std::collections::{HashMap, HashSet};

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input6.txt").unwrap();
    println!(
        "cycles before infinite loop: {}",
        find_cycles_to_infinite_loop::<16>(&input)
    );
    println!(
        "cycles of infinite loop: {}",
        find_cycles_of_infinite_loop::<16>(&input)
    );
}

fn find_cycles_to_infinite_loop<const T: usize>(input: &str) -> usize {
    let mut already_seen = HashSet::new();
    let mut memory_banks = MemoryBanks::<T>::new(input);
    while !already_seen.contains(&memory_banks) {
        already_seen.insert(memory_banks.clone());
        memory_banks.reallocate();
    }
    already_seen.len()
}

fn find_cycles_of_infinite_loop<const T: usize>(input: &str) -> usize {
    let mut already_seen = HashMap::new();
    let mut memory_banks = MemoryBanks::<T>::new(input);
    let mut i = 0usize;
    while !already_seen.contains_key(&memory_banks) {
        already_seen.insert(memory_banks.clone(), i);
        i += 1;
        memory_banks.reallocate();
    }
    already_seen.len() - already_seen.get(&memory_banks).unwrap()
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct MemoryBanks<const T: usize>([u32; T]);

impl<const T: usize> MemoryBanks<T> {
    pub fn new(input: &str) -> Self {
        Self(
            input
                .split_whitespace()
                .map(|val| val.parse().expect("parse error"))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
    fn get_bank_with_most_blocks(&self) -> usize {
        let max = *self.0.iter().max().unwrap();
        self.0
            .iter()
            .enumerate()
            .filter(|(_, &val)| val == max)
            .next()
            .unwrap()
            .0
    }
    pub fn reallocate(&mut self) {
        let mut bank = self.get_bank_with_most_blocks();
        let blocks = std::mem::take(&mut self.0[bank]);
        for _ in 0..blocks {
            bank = (bank + 1) % self.0.len();
            self.0[bank] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(find_cycles_to_infinite_loop::<4>("0 2 7 0"), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(find_cycles_of_infinite_loop::<4>("0 2 7 0"), 4);
    }
}
