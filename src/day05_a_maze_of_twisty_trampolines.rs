use std::str::FromStr;

use log::debug;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input5.txt").unwrap();
    println!("steps to exit: {}", run_maze(&input, false));
    println!(
        "steps to exit with strange steps: {}",
        run_maze(&input, true)
    );
}

fn run_maze(input: &str, strange_jumps: bool) -> usize {
    let mut maze: Maze = input.parse().unwrap();
    maze.strange_jumps = strange_jumps;
    let mut steps = 0;
    loop {
        steps += 1;
        if !maze.next() {
            break;
        }
    }
    steps
}

struct Maze {
    position: usize,
    instructions: Vec<i32>,
    strange_jumps: bool,
}

impl Maze {
    pub fn new(instructions: Vec<i32>) -> Maze {
        Maze {
            position: 0,
            instructions,
            strange_jumps: false,
        }
    }
    pub fn next(&mut self) -> bool {
        let jump = self.instructions[self.position];
        let next_position = self.position as i32 + jump;
        if self.strange_jumps && jump >= 3 {
            self.instructions[self.position] -= 1;
        } else {
            self.instructions[self.position] += 1;
        }
        debug!(
            "jumped from position {} to {}",
            self.position, next_position
        );
        if next_position < 0 {
            return false;
        } else if next_position >= self.instructions.len() as i32 {
            return false;
        }
        self.position = next_position as usize;
        true
    }
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.lines()
                .map(|line| line.parse().expect("parse error"))
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "0
3
0
1
-3";
        assert_eq!(run_maze(input, false), 5);
    }

    #[test]
    fn test2() {
        let input = "0
3
0
1
-3";
        assert_eq!(run_maze(input, true), 10);
    }
}