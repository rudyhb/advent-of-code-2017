use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
use std::str::FromStr;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input22.txt").unwrap();
    let mut grid: Grid = input.parse().unwrap();
    grid.steps(10_000);
    println!(
        "bursts causing infection: {}",
        grid.bursts_causing_infection
    );

    let grid: Grid = input.parse().unwrap();
    let mut grid: GridV2 = grid.into();
    grid.steps(10_000_000);
    println!(
        "bursts causing infection v2: {}",
        grid.bursts_causing_infection
    );
}

struct Grid {
    infected: HashSet<Coord>,
    direction: Direction,
    position: Coord,
    bursts_causing_infection: usize,
    step: usize,
}

impl Grid {
    pub fn steps(&mut self, count: usize) {
        for _ in 0..count {
            // println!("step {}\n{}\n", self.step, self);
            // println!("press enter to continue");
            // std::io::stdin().read_line(&mut String::new()).unwrap();

            self.step();
        }
    }
    fn step(&mut self) {
        if self.infected.contains(&self.position) {
            self.infected.remove(&self.position);
            self.direction = self.direction.turn_right();
        } else {
            self.infected.insert(self.position.clone());
            self.direction = self.direction.turn_left();

            self.bursts_causing_infection += 1;
        }
        self.position += self.direction.into();
        self.step += 1;
    }
}

enum NodeState {
    Weakened,
    Infected,
    Flagged,
}

struct GridV2 {
    node_states: HashMap<Coord, NodeState>,
    direction: Direction,
    position: Coord,
    bursts_causing_infection: usize,
    step: usize,
}

impl GridV2 {
    pub fn steps(&mut self, count: usize) {
        for _ in 0..count {
            self.step();
        }
    }
    fn step(&mut self) {
        match self.node_states.get(&self.position) {
            None => {
                self.node_states
                    .insert(self.position.clone(), NodeState::Weakened);
                self.direction = self.direction.turn_left();
            }
            Some(NodeState::Weakened) => {
                self.node_states
                    .insert(self.position.clone(), NodeState::Infected);
                self.bursts_causing_infection += 1;
            }
            Some(NodeState::Infected) => {
                self.node_states
                    .insert(self.position.clone(), NodeState::Flagged);
                self.direction = self.direction.turn_right();
            }
            Some(NodeState::Flagged) => {
                self.node_states.remove(&self.position);
                self.direction = self.direction.turn_right().turn_right();
            }
        }
        self.position += self.direction.into();
        self.step += 1;
    }
}

impl From<Grid> for GridV2 {
    fn from(value: Grid) -> Self {
        Self {
            node_states: value
                .infected
                .into_iter()
                .map(|c| (c, NodeState::Infected))
                .collect(),
            direction: value.direction,
            position: value.position,
            bursts_causing_infection: value.bursts_causing_infection,
            step: value.step,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self
            .infected
            .iter()
            .map(|c| c.y.abs().max(c.x.abs()))
            .max()
            .unwrap_or(0);
        write!(
            f,
            "{}",
            (-n..=n)
                .map(|y| (-n..=n)
                    .map(|x| {
                        let c = Coord::new(x, y);
                        let current = self.position == c;
                        format!(
                            "{}{}{}",
                            if current { '[' } else { ' ' },
                            if self.infected.contains(&c) { '#' } else { '.' },
                            if current { ']' } else { ' ' },
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(""))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Coord { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Coord::new(0, -1),
            Direction::Down => Coord::new(0, 1),
            Direction::Left => Coord::new(-1, 0),
            Direction::Right => Coord::new(1, 0),
        }
    }
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = (s.lines().count() / 2) as i64;
        let mut infected = HashSet::new();
        for (y, row) in s.lines().enumerate() {
            for (x, _) in row.chars().enumerate().filter(|(_, c)| *c == '#') {
                infected.insert(Coord::new(x as i64 - n, y as i64 - n));
            }
        }
        Ok(Grid {
            infected,
            direction: Direction::Up,
            position: Coord::new(0, 0),
            bursts_causing_infection: 0,
            step: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "..#
#..
...";
        let mut grid: Grid = input.parse().unwrap();
        grid.steps(7);
        assert_eq!(5, grid.bursts_causing_infection);
        grid.steps(70 - 7);
        assert_eq!(41, grid.bursts_causing_infection);
        grid.steps(10_000 - 70);
        assert_eq!(5587, grid.bursts_causing_infection);
    }
    #[test]
    fn test2() {
        let input = "..#
#..
...";
        let grid: Grid = input.parse().unwrap();
        let mut grid: GridV2 = grid.into();
        grid.steps(100);
        assert_eq!(26, grid.bursts_causing_infection);
        grid.steps(10_000_000 - 100);
        assert_eq!(2511944, grid.bursts_causing_infection);
    }
}
