use std::ops::AddAssign;
use std::str::FromStr;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input11.txt").unwrap();
    let (steps, max_steps) = get_distance_steps(&input);
    println!("child process is {} steps away", steps);
    println!("child process was max {} steps away", max_steps);
}

fn get_distance_steps(input: &str) -> (u32, u32) {
    // n/s moves +2y/-2y
    // ne moves +1y +1x, etc
    let mut position = Coord::default();
    let mut max_distance = 0;
    for direction in input.split(',').map(|s| s.parse::<Direction>().unwrap()) {
        position += Coord::from(direction);
        max_distance = max_distance.max(position.hex_distance());
    }

    (position.hex_distance(), max_distance)
}

#[derive(Default, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn hex_distance(&self) -> u32 {
        let y = self.y - self.x.abs() * (if self.y.is_positive() { 1 } else { -1 });
        (self.x.abs() + y.abs() / 2) as u32
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl From<Direction> for Coord {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => Coord::new(0, 2),
            Direction::NorthEast => Coord::new(1, 1),
            Direction::SouthEast => Coord::new(1, -1),
            Direction::South => Coord::new(0, -2),
            Direction::SouthWest => Coord::new(-1, -1),
            Direction::NorthWest => Coord::new(-1, 1),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "n" => Self::North,
            "ne" => Self::NorthEast,
            "se" => Self::SouthEast,
            "s" => Self::South,
            "sw" => Self::SouthWest,
            "nw" => Self::NorthWest,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(get_distance_steps("ne,ne,ne").0, 3);
    }
    #[test]
    fn test2() {
        assert_eq!(get_distance_steps("ne,ne,sw,sw").0, 0);
    }
    #[test]
    fn test3() {
        assert_eq!(get_distance_steps("ne,ne,s,s").0, 2);
    }
    #[test]
    fn test4() {
        assert_eq!(get_distance_steps("se,sw,se,sw,sw").0, 3);
    }
}
