use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{bail, Context};
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input25.txt").unwrap();
    let mut machine: Machine = input.parse().unwrap();
    println!("diagnostic checksum: {}", machine.run_until_checksum());
}

struct Machine {
    state: char,
    tape: Tape,
    states: HashMap<char, State>,
    checksum_after: u64,
}

impl Machine {
    pub fn run_until_checksum(&mut self) -> usize {
        for step in 0..self.checksum_after {
            let state = self
                .states
                .get(&self.state)
                .with_context(|| format!("cannot find state '{}' at step {}", self.state, step))
                .unwrap();
            self.state = state.execute(&mut self.tape);
        }
        self.tape.count_set()
    }
}

struct Tape {
    cursor: i32,
    vec_right: Vec<bool>,
    vec_left: Vec<bool>,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            vec_right: vec![false],
            vec_left: vec![],
        }
    }
    fn move_left(&mut self) {
        self.cursor -= 1;
        if self.cursor < -(self.vec_left.len() as i32) {
            self.vec_left.push(false);
        }
    }
    fn move_right(&mut self) {
        self.cursor += 1;
        if self.cursor >= self.vec_right.len() as i32 {
            self.vec_right.push(false);
        }
    }
    pub fn move_to(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.move_right(),
            Direction::Left => self.move_left(),
        }
    }
    pub fn get(&self) -> bool {
        if self.cursor < 0 {
            self.vec_left[(self.cursor.abs() - 1) as usize]
        } else {
            self.vec_right[self.cursor as usize]
        }
    }
    pub fn get_mut(&mut self) -> &mut bool {
        if self.cursor < 0 {
            &mut self.vec_left[(self.cursor.abs() - 1) as usize]
        } else {
            &mut self.vec_right[self.cursor as usize]
        }
    }
    pub fn count_set(&self) -> usize {
        self.vec_right
            .iter()
            .chain(self.vec_left.iter())
            .filter(|val| **val)
            .count()
    }
}

#[derive(Debug)]
struct State {
    conditions: [(Condition, StateAction); 2],
}

impl State {
    pub fn execute(&self, tape: &mut Tape) -> char {
        let action = self
            .conditions
            .iter()
            .filter(|(condition, _)| condition.evaluate(tape))
            .map(|(_, action)| action)
            .next()
            .context("no matching condition")
            .unwrap();
        *tape.get_mut() = action.write_value;
        tape.move_to(action.move_to);

        action.continue_with
    }
}

#[derive(Debug)]
struct Condition(bool);

impl Condition {
    pub fn evaluate(&self, tape: &Tape) -> bool {
        self.0 == tape.get()
    }
}

#[derive(Debug)]
struct StateAction {
    write_value: bool,
    move_to: Direction,
    continue_with: char,
}

impl StateAction {
    pub fn new(write_value: bool, move_to: Direction, continue_with: char) -> Self {
        Self {
            write_value,
            move_to,
            continue_with,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "left" => Self::Left,
            "right" => Self::Right,
            other => bail!("cannot parse Direction '{}'", other),
        })
    }
}

impl FromStr for Condition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref CONDITION: Regex = Regex::new(r#"If the current value is (\d+):"#).unwrap();
        }
        let value = CONDITION
            .captures_iter(s)
            .next()
            .with_context(|| format!("invalid condition '{}'", s))?
            .get(1)
            .context("get condition val")?
            .as_str();
        Ok(Condition(parse_int_to_bool(value)?))
    }
}

fn parse_int_to_bool(s: &str) -> anyhow::Result<bool> {
    Ok(match s.parse::<i32>()? {
        0 => false,
        1 => true,
        other => bail!("invalid bool '{}'", other),
    })
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BEGIN: Regex = Regex::new(r#"Begin in state ([A-Z])."#).unwrap();
            static ref CHECK: Regex =
                Regex::new(r#"Perform a diagnostic checksum after (\d+) steps."#).unwrap();
        }
        let mut lines = s.lines();
        let state = BEGIN
            .captures_iter(lines.next().context("machine line 1 not found")?)
            .next()
            .context("invalid machine line 1")?
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap();
        let checksum_after: u64 = CHECK
            .captures_iter(lines.next().context("machine line 2 not found")?)
            .next()
            .context("invalid machine line 2")?
            .get(1)
            .unwrap()
            .as_str()
            .parse()?;

        let mut states = HashMap::new();
        while let Some((c, state)) = try_parse_state(&mut lines)? {
            debug!("got state {}: {:?}", c, state);
            states.insert(c, state);
        }

        Ok(Self {
            state,
            tape: Tape::new(),
            states,
            checksum_after,
        })
    }
}

fn try_parse_state<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
) -> anyhow::Result<Option<(char, State)>> {
    if lines.next().is_none() {
        return Ok(None);
    }
    lazy_static! {
        static ref STATE: Regex = Regex::new(r#"In state ([A-Z]):"#).unwrap();
    }
    let line = lines.next().context("missing state line")?;
    let state_name = STATE
        .captures_iter(line)
        .next()
        .with_context(|| format!("invalid state line 1: '{}'", line))?
        .get(1)
        .unwrap()
        .as_str()
        .chars()
        .next()
        .unwrap();
    fn get_action_value(s: &str) -> anyhow::Result<&str> {
        Ok(s.split_whitespace()
            .last()
            .context("invalid action")?
            .trim_end_matches('.'))
    }
    let conditions = (0..2)
        .map(|_| {
            let condition: Condition = lines.next().context("condition not found")?.parse()?;
            let actions = StateAction::new(
                parse_int_to_bool(get_action_value(
                    lines.next().context("not enough actions")?,
                )?)?,
                get_action_value(lines.next().context("not enough actions")?)?.parse()?,
                get_action_value(lines.next().context("not enough actions")?)?
                    .chars()
                    .next()
                    .context("invalid state")?,
            );
            Ok((condition, actions))
        })
        .collect::<anyhow::Result<Vec<(Condition, StateAction)>>>()?
        .try_into()
        .unwrap();
    Ok(Some((state_name, State { conditions })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

        let mut machine: Machine = input.parse().unwrap();
        assert_eq!(3, machine.run_until_checksum());
    }
}
