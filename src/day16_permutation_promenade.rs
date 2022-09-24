use std::collections::HashMap;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input16.txt").unwrap();
    let mut programs = Programs::new(16);
    let instructions: Vec<Instruction> = input.split(',').map(|i| i.parse().unwrap()).collect();
    for instruction in instructions.iter() {
        programs.dance(instruction);
    }
    println!("order after dance: {}", programs);

    let mut seen = HashMap::new();
    seen.insert(programs.clone(), 0u32);

    let mut i = 0u32;
    let (first, second) = loop {
        i += 1;
        for instruction in instructions.iter() {
            programs.dance(instruction);
        }
        if let Some(existing) = seen.get(&programs) {
            break (*existing, i);
        } else {
            seen.insert(programs.clone(), i);
        }
    };
    println!("seen after dances {} and {}: {}", first, second, programs);

    let billionth_dance = (1_000_000_000 - first - 1) % (second - first);
    let value = seen.iter().find(|val| *val.1 == billionth_dance).unwrap().0;
    println!("programs after billionth dance: {}", value);
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Programs(Vec<u8>);

impl Programs {
    pub fn new(len: u8) -> Self {
        Self((0..len).map(|i| 'a' as u8 + i).collect())
    }
    pub fn dance(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Spin(len) => {
                let left = self.0[..self.0.len() - len].to_vec();
                let mut right = Vec::with_capacity(self.0.len());
                right.extend(&self.0[self.0.len() - len..]);
                right.extend(left);
                self.0 = right;
            }
            Instruction::Exchange(left, right) => {
                self.0.swap(*left, *right);
            }
            Instruction::Partner(_, _) => {
                self.dance(&instruction.partner_to_exchange(&self));
            }
        }
    }
}

impl std::fmt::Display for Programs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .copied()
                .map(|val| val as char)
                .collect::<String>()
        )
    }
}

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

impl Instruction {
    pub fn partner_to_exchange(&self, programs: &Programs) -> Self {
        if let Self::Partner(left, right) = self {
            let l = programs.0.iter().position(|val| val == left).unwrap();
            let r = programs.0.iter().position(|val| val == right).unwrap();
            Self::Exchange(l, r)
        } else {
            panic!("invalid operation");
        }
    }
}

#[derive(Debug)]
enum ParseInstructionError {
    NumericParseError,
    InvalidInputLength,
    InvalidFormat,
}

impl std::str::FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        Ok(
            match chars
                .next()
                .ok_or(ParseInstructionError::InvalidInputLength)?
            {
                's' => Self::Spin(
                    s[1..]
                        .parse()
                        .map_err(|_| ParseInstructionError::NumericParseError)?,
                ),
                'x' => {
                    let mut parts = s[1..].split('/');
                    Self::Exchange(
                        parts
                            .next()
                            .ok_or(ParseInstructionError::InvalidInputLength)?
                            .parse()
                            .map_err(|_| ParseInstructionError::NumericParseError)?,
                        parts
                            .next()
                            .ok_or(ParseInstructionError::InvalidInputLength)?
                            .parse()
                            .map_err(|_| ParseInstructionError::NumericParseError)?,
                    )
                }
                'p' => {
                    let left = chars
                        .next()
                        .ok_or(ParseInstructionError::InvalidInputLength)?;
                    chars.next();
                    let right = chars
                        .next()
                        .ok_or(ParseInstructionError::InvalidInputLength)?;
                    Self::Partner(left as u8, right as u8)
                }
                _ => return Err(ParseInstructionError::InvalidFormat),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "s1,x3/4,pe/b";
        let mut programs = Programs::new(5);
        for instruction in input.split(',').map(|i| i.parse::<Instruction>().unwrap()) {
            programs.dance(&instruction);
        }
        assert_eq!(format!("{}", programs), "baedc");
    }
}
