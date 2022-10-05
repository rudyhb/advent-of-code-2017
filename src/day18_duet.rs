use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use log::{debug, info};

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input18.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let cpu = Cpu::new(&instructions, 0);

    println!(
        "first sound recovered: {}",
        return_first_sound_recovered(cpu)
    );

    let mut numbers = get_numbers(&instructions);
    println!("the 'duet' is sorting {} numbers", numbers.len());
    info!("numbers: {:?}", numbers);
    let rounds = sort(&mut numbers);
    println!("number of rounds to sort: {}", rounds);
    println!(
        "program 1 has to send values {} times",
        (rounds / 2 + 1) * (127)
    );
    // println!(
    //     "after duet, program1 sent {} values",
    //     run_duet(&instructions)
    // );
}

fn return_first_sound_recovered(mut cpu: Cpu) -> i64 {
    let mut recovered = false;
    let mut last_sound_played = 0i64;
    while !recovered {
        if !cpu.run(
            |sound: i64| {
                last_sound_played = sound;
            },
            |current_val: i64| {
                if current_val != 0 {
                    recovered = true;
                }
            },
        ) {
            panic!("ran out of instructions");
        }
    }
    last_sound_played
}

fn get_numbers(instructions: &[Instruction]) -> Vec<i64> {
    let mut program = Program::new("0", instructions, 0);
    let mut numbers = Vec::with_capacity(128);
    while !program.is_stuck() {
        let (ok, sent) = program.run();
        assert!(ok);
        if let Some(val) = sent {
            numbers.push(val);
        }
    }
    numbers
}

fn sort(values: &mut [i64]) -> u32 {
    for i in 1.. {
        for j in 0..values.len() - 1 {
            if values[j] < values[j + 1] {
                values.swap(j, j + 1);
            }
        }

        if values.windows(2).all(|val| val[0] >= val[1]) {
            return i;
        }
    }
    panic!()
}

#[allow(unused)]
fn run_duet(instructions: &[Instruction]) -> u32 {
    let mut program1 = Program::new("0", instructions, 0);
    let mut program2 = Program::new("1", instructions, 1);
    loop {
        let (running1, sent) = program1.run();
        if let Some(val) = sent {
            program2.receive_queue.push_back(val);
        }
        let (running2, sent) = program2.run();
        if let Some(val) = sent {
            program1.receive_queue.push_back(val);
        }
        if !running1 && !running2 {
            break;
        }
        if program1.is_stuck() && program2.is_stuck() {
            break;
        }
        // debug!("program1: {:?}\n\nprogram2: {:?}\n\n\n", program1, program2);
        debug!("cpu1: {:?}\n\ncpu2: {:?}\n\n\n", program1.cpu, program2.cpu);
    }
    program1.num_sent
}

#[derive(Debug)]
struct Program<'a> {
    name: &'static str,
    cpu: Cpu<'a>,
    receive_queue: VecDeque<i64>,
    is_waiting: bool,
    num_sent: u32,
}

impl<'a> Program<'a> {
    pub fn new(name: &'static str, instructions: &'a [Instruction], id: i64) -> Self {
        Self {
            name,
            cpu: Cpu::new(instructions, id),
            receive_queue: Default::default(),
            is_waiting: false,
            num_sent: 0,
        }
    }
    pub fn run(&mut self) -> (bool, Option<i64>) {
        if self.is_waiting {
            self.try_recover();
            return (true, None);
        }

        let mut wait = false;
        let mut sent_value = None;
        let running = self.cpu.run(
            |value_sent| {
                sent_value = Some(value_sent);
            },
            |_: i64| {
                wait = true;
            },
        );
        if wait {
            self.wait();
        }
        if sent_value.is_some() {
            self.num_sent += 1;
        }
        (running, sent_value)
    }
    pub fn wait(&mut self) {
        self.is_waiting = true;
    }
    pub fn try_recover(&mut self) {
        if let Some(val) = self.receive_queue.pop_front() {
            info!("program {} received {}", self.name, val);
            self.cpu.recover(val);
            self.is_waiting = false;
        }
    }
    pub fn is_stuck(&self) -> bool {
        self.is_waiting && self.receive_queue.is_empty()
    }
}

struct Cpu<'a> {
    instructions: &'a [Instruction],
    current_position: i64,
    registers: HashMap<char, i64>,
    register_to_recover: char,
}

impl<'a> Debug for Cpu<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "current_pos: {}, registers: {:?}",
            self.current_position, self.registers
        )
    }
}

impl<'a> Cpu<'a> {
    pub fn new(instructions: &'a [Instruction], id: i64) -> Self {
        let mut registers: HashMap<char, i64> = HashMap::new();
        registers.insert('p', id);

        Self {
            instructions,
            current_position: 0,
            registers,
            register_to_recover: 'a',
        }
    }
    pub fn run(&mut self, mut on_send: impl FnMut(i64), mut on_recover: impl FnMut(i64)) -> bool {
        if self.current_position < 0 || self.current_position >= self.instructions.len() as i64 {
            return false;
        }

        let instruction = &self.instructions[self.current_position as usize];
        let mut jmp = 1;
        match instruction {
            Instruction::Set(r, val) => {
                self.registers.insert(*r, val.get(&self.registers));
            }
            Instruction::Add(r, val) => {
                *self.registers.entry(*r).or_default() += val.get(&self.registers);
            }
            Instruction::Multiply(r, val) => {
                *self.registers.entry(*r).or_default() *= val.get(&self.registers);
            }
            Instruction::Mod(r, val) => {
                // let previous = self.registers.get(r).copied().unwrap_or_default();
                // let modulus = val.get(&self.registers);
                // self.registers.insert(*r, ((previous % modulus) + modulus) % modulus);
                *self.registers.entry(*r).or_default() %= val.get(&self.registers);
            }
            Instruction::Send(r) => {
                on_send(self.registers.get(r).copied().unwrap_or_default());
            }
            Instruction::Recover(r) => {
                let current_val = self.registers.get(r).copied().unwrap_or_default();
                self.register_to_recover = *r;
                on_recover(current_val);
            }
            Instruction::JumpIfGreaterThanZero(r, val) => {
                if self.registers.get(r).copied().unwrap_or_default() > 0 {
                    jmp = val.get(&self.registers);
                }
            }

            Instruction::MultiplyByPowerOf2(r, val) => {
                *self.registers.entry(*r).or_default() *= 2i64.pow(val.get(&self.registers) as u32);
            }
            Instruction::NoOperation => {}
        }

        self.current_position += jmp;
        debug!("{:?} -> {:?}", instruction, self);
        true
    }
    pub fn recover(&mut self, val: i64) {
        self.registers.insert(self.register_to_recover, val);
    }
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
    Add(char, RegisterOrValue),
    Multiply(char, RegisterOrValue),
    Mod(char, RegisterOrValue),
    Send(char),
    Recover(char),
    JumpIfGreaterThanZero(char, RegisterOrValue),
    MultiplyByPowerOf2(char, RegisterOrValue),
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
            "add" => Self::Add(
                parse_char(parts.next().unwrap()),
                parse(parts.next().unwrap()),
            ),
            "mul" => Self::Multiply(
                parse_char(parts.next().unwrap()),
                parse(parts.next().unwrap()),
            ),
            "mod" => Self::Mod(
                parse_char(parts.next().unwrap()),
                parse(parts.next().unwrap()),
            ),
            "snd" => Self::Send(parse_char(parts.next().unwrap())),
            "rcv" => Self::Recover(parse_char(parts.next().unwrap())),
            "jgz" => Self::JumpIfGreaterThanZero(
                parse_char(parts.next().unwrap()),
                parse(parts.next().unwrap()),
            ),

            "mulpow2" => Self::MultiplyByPowerOf2(
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
        let input = "set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2";
        let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
        let cpu = Cpu::new(&instructions, 0);

        assert_eq!(return_first_sound_recovered(cpu), 4);
    }
}
