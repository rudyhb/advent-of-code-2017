use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use log::debug;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input23mod.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut debugger = Debugger::new();
    let mut cpu = Cpu::new(&mut debugger, &instructions);
    while cpu.run() {}

    println!("times mul invoked: {}", debugger.times_mul_invoked);

    let mut debugger = Debugger::new();
    let mut cpu = Cpu::new(&mut debugger, &instructions);
    cpu.set_register('a', 1);
    while cpu.run() {}

    println!("value at h: {}", cpu.get_register('h'));
}

struct Debugger {
    pub times_mul_invoked: u64,
    states: HashSet<State>,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            times_mul_invoked: 0,
            states: HashSet::new(),
        }
    }
    pub fn update_states(&mut self, current_position: i64, registers: &HashMap<char, i64>) {
        let state = State::from(current_position, registers);
        if !self.states.insert(state.clone()) {
            panic!("infinite loop found: {:?}", state);
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    current_position: i64,
    registers: [i64; 8],
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line {} with values: {{\n{}}}",
            self.current_position + 1,
            self.registers
                .iter()
                .enumerate()
                .map(|(i, val)| { format!("{}: {}", (i as u8 + 'a' as u8) as char, val) })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl State {
    pub fn from(current_position: i64, registers: &HashMap<char, i64>) -> Self {
        Self {
            current_position,
            registers: ('a'..='h')
                .map(|c| registers.get(&c).copied().unwrap_or_default())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
    #[allow(unused)]
    pub fn update_register(&mut self, char: char, value: i64) {
        self.registers[(char as u8 - 'a' as u8) as usize] = value;
    }
}

struct Cpu<'a, 'b> {
    debugger: &'b mut Debugger,
    instructions: &'a [Instruction],
    current_position: i64,
    registers: HashMap<char, i64>,
}

impl<'a, 'b> Debug for Cpu<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "current_pos: {}, registers: {:?}",
            self.current_position, self.registers
        )
    }
}

impl<'a, 'b> Cpu<'a, 'b> {
    pub fn new(debugger: &'b mut Debugger, instructions: &'a [Instruction]) -> Self {
        Self {
            debugger,
            instructions,
            current_position: 0,
            registers: HashMap::new(),
        }
    }
    pub fn set_register(&mut self, r: char, val: i64) {
        self.registers.insert(r, val);
    }
    pub fn get_register(&mut self, r: char) -> i64 {
        self.registers.get(&r).copied().unwrap_or_default()
    }
    pub fn run(&mut self) -> bool {
        if self.current_position < 0 || self.current_position >= self.instructions.len() as i64 {
            return false;
        }

        let instruction = &self.instructions[self.current_position as usize];
        let mut jmp = 1;
        match instruction {
            Instruction::Set(r, val) => {
                self.registers.insert(*r, val.get(&self.registers));
            }
            Instruction::Sub(r, val) => {
                *self.registers.entry(*r).or_default() -= val.get(&self.registers);
            }
            Instruction::Multiply(r, val) => {
                *self.registers.entry(*r).or_default() *= val.get(&self.registers);
                self.debugger.times_mul_invoked += 1;
            }
            Instruction::JumpIfNotZero(check, val) => {
                if check.get(&self.registers) != 0 {
                    jmp = val.get(&self.registers);
                }
            }
            Instruction::SetIsPrime(r, val) => {
                let val = val.get(&self.registers);
                self.registers
                    .insert(*r, if is_prime(val as usize) { 1 } else { 0 });
                self.debugger.times_mul_invoked += (val - 2).pow(2) as u64;
            }
            Instruction::NoOperation => {}
        }

        self.current_position += jmp;
        debug!("{:?} -> {:?}", instruction, self);
        self.debugger
            .update_states(self.current_position, &self.registers);
        true
    }
}

fn is_prime(n: usize) -> bool {
    let mut not_prime = vec![false; n + 1];
    let mut last = 1;
    while last < n {
        let p = if let Some(p) = not_prime
            .iter()
            .enumerate()
            .skip(last + 1)
            .filter(|(_, &marked)| !marked)
            .map(|(p, _)| p)
            .next()
        {
            p
        } else {
            break;
        };
        for i in 2..=n / p {
            not_prime[i * p] = true;
        }
        last = p;
    }

    !not_prime[n]
}

#[derive(Debug)]
enum RegisterOrValue {
    Register(char),
    Value(i64),
}

impl RegisterOrValue {
    pub fn get(&self, registers: &HashMap<char, i64>) -> i64 {
        match self {
            RegisterOrValue::Register(r) => registers.get(r).copied().unwrap_or_default(),
            RegisterOrValue::Value(v) => *v,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Set(char, RegisterOrValue),
    Sub(char, RegisterOrValue),
    Multiply(char, RegisterOrValue),
    JumpIfNotZero(RegisterOrValue, RegisterOrValue),
    SetIsPrime(char, RegisterOrValue),
    NoOperation,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(s: &str) -> RegisterOrValue {
            if let Ok(val) = s.parse::<i64>() {
                RegisterOrValue::Value(val)
            } else {
                RegisterOrValue::Register(parse_char(s))
            }
        }
        fn parse_char(s: &str) -> char {
            if s.len() == 1 {
                s.chars().next().unwrap()
            } else {
                panic!("parse error");
            }
        }

        let mut parts = s.split_whitespace();
        Ok(match parts.next().unwrap() {
            "set" => Self::Set(
                parse_char(parts.next().unwrap()),
                parse(parts.next().unwrap()),
            ),
            "sub" => Self::Sub(
                parse_char(parts.next().unwrap()),
                parse(parts.next().unwrap()),
            ),
            "mul" => Self::Multiply(
                parse_char(parts.next().unwrap()),
                parse(parts.next().unwrap()),
            ),
            "jnz" => {
                Self::JumpIfNotZero(parse(parts.next().unwrap()), parse(parts.next().unwrap()))
            }
            "set_is_prime" => Self::SetIsPrime(
                parse_char(parts.next().unwrap()),
                parse(parts.next().unwrap()),
            ),
            "noop" => Self::NoOperation,
            other => panic!("invalid instruction: {}", other),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        env_logger::init();
        let input = std::fs::read_to_string("input/input23mod.txt").unwrap();
        let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
        let mut debugger = Debugger::new();
        let cpu = Cpu::new(&mut debugger, &instructions);

        assert_eq!(cpu.instructions.len(), 32);
    }

    #[test]
    fn test_primes() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(17));
        assert!(is_prime(53));
        assert!(is_prime(1013));

        assert!(!is_prime(4));
        assert!(!is_prime(3 * 17));
        assert!(!is_prime(17 * 59));
        assert!(!is_prime(42 * 63));
    }
}
