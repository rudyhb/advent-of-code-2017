use std::collections::HashMap;

use instruction::Instruction;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input8.txt").unwrap();
    let mut registers = Registers::new();
    let max_intermediate = process(&mut registers, &input);
    println!("max final value of any register is {}", registers.max());
    println!(
        "max intermediate value of any register is {}",
        max_intermediate
    );
}

fn process<'a, 'b>(registers: &'b mut Registers<'a>, instructions: &'a str) -> i32 {
    let mut max = 0i32;
    for instruction_str in instructions.lines() {
        let instruction: Instruction = instruction_str.into();
        instruction.process(registers);
        max = max.max(registers.max());
    }
    max
}

struct Registers<'a>(HashMap<&'a str, i32>);

impl<'a> Registers<'a> {
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn max(&self) -> i32 {
        self.0.iter().map(|(_, &val)| val).max().unwrap_or(0)
    }
    pub fn get(&self, register: &'a str) -> i32 {
        self.0.get(register).copied().unwrap_or(0)
    }
    pub fn get_mut(&mut self, register: &'a str) -> &mut i32 {
        self.0.entry(register).or_default()
    }
}

mod instruction {
    use std::str::SplitWhitespace;

    use crate::day08_i_heard_you_like_registers::Registers;

    enum IncreaseDecrease {
        Increase,
        Decrease,
    }
    enum Operator {
        EqualTo,
        NotEqualTo,
        GreaterThan,
        LessThan,
        GreaterThanOrEqualTo,
        LessThanOrEqualTo,
    }
    struct Condition<'a> {
        register: &'a str,
        operator: Operator,
        value: i32,
    }
    pub(super) struct Instruction<'a> {
        register: &'a str,
        increase_decrease: IncreaseDecrease,
        value: i32,
        condition: Condition<'a>,
    }

    impl<'a> Condition<'a> {
        fn is_true(&self, registers: &Registers) -> bool {
            let lhs = registers.get(self.register);
            let rhs = self.value;
            match self.operator {
                Operator::EqualTo => lhs == rhs,
                Operator::NotEqualTo => lhs != rhs,
                Operator::GreaterThan => lhs > rhs,
                Operator::LessThan => lhs < rhs,
                Operator::GreaterThanOrEqualTo => lhs >= rhs,
                Operator::LessThanOrEqualTo => lhs <= rhs,
            }
        }
    }

    impl<'a> From<SplitWhitespace<'a>> for Condition<'a> {
        fn from(mut parts: SplitWhitespace<'a>) -> Self {
            parts.next();
            let register = parts.next().unwrap().trim();
            let operator = match parts.next().unwrap().trim() {
                "==" => Operator::EqualTo,
                "!=" => Operator::NotEqualTo,
                ">" => Operator::GreaterThan,
                "<" => Operator::LessThan,
                ">=" => Operator::GreaterThanOrEqualTo,
                "<=" => Operator::LessThanOrEqualTo,
                other => panic!("invalid operator: '{}'", other),
            };
            let value: i32 = parts.next().unwrap().trim().parse().unwrap();
            Self {
                register,
                operator,
                value,
            }
        }
    }

    impl<'a> Instruction<'a> {
        pub fn process<'b>(&'b self, registers: &'b mut Registers<'a>) {
            if self.condition.is_true(registers) {
                let register = registers.get_mut(self.register);
                match self.increase_decrease {
                    IncreaseDecrease::Increase => {
                        *register += self.value;
                    }
                    IncreaseDecrease::Decrease => {
                        *register -= self.value;
                    }
                }
            }
        }
    }

    impl<'a> From<&'a str> for Instruction<'a> {
        fn from(s: &'a str) -> Self {
            let mut parts = s.split_whitespace();
            let register = parts.next().unwrap().trim();
            let increase_decrease = match parts.next().unwrap().trim() {
                "inc" => IncreaseDecrease::Increase,
                "dec" => IncreaseDecrease::Decrease,
                other => panic!("invalid increase/decrease instruction: '{}'", other),
            };
            let value: i32 = parts.next().unwrap().trim().parse().unwrap();
            let condition: Condition = parts.into();
            Self {
                register,
                increase_decrease,
                value,
                condition,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        let mut registers = Registers::new();
        process(&mut registers, &input);
        assert_eq!(registers.max(), 1);
    }

    #[test]
    fn test2() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        let mut registers = Registers::new();
        let max = process(&mut registers, &input);
        assert_eq!(max, 10);
    }
}
