use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input19.txt").unwrap();
    let path = get_path(&input);
    println!("obtained path: {}", path);
    println!("path traversed: {}", get_path_traveled(&path));
    println!("steps taken: {}", path.0.len());
}

fn get_path(input: &str) -> Path {
    input.parse().unwrap()
}

fn get_path_traveled(path: &Path) -> String {
    path.0
        .iter()
        .filter_map(|section| {
            if let Section::Letter(c) = section {
                Some(c)
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug)]
enum Section {
    Vertical,
    Horizontal,
    Cross,
    Letter(char),
}

struct Path(Vec<Section>);

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|section| match section {
                    Section::Vertical => '|',
                    Section::Horizontal => '-',
                    Section::Cross => '+',
                    Section::Letter(c) => *c,
                })
                .collect::<String>()
        )
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    pub fn traverse(&self, col: &mut usize, row: &mut usize) {
        match self {
            Direction::Up => *row -= 1,
            Direction::Down => *row += 1,
            Direction::Left => *col -= 1,
            Direction::Right => *col += 1,
        }
    }
}

#[derive(Debug)]
struct Matrix(Vec<Vec<char>>);

impl Matrix {
    pub fn new(input: Vec<Vec<char>>) -> Self {
        Self(input)
    }
    pub fn col_where(&self, row: usize, f: impl Fn(char) -> bool) -> Option<usize> {
        self.0[row].iter().position(|&c| f(c))
    }
    pub fn try_get(&self, row: usize, col: usize) -> Option<char> {
        self.0.get(row).and_then(|row| row.get(col)).copied()
    }
    pub fn get(&self, row: usize, col: usize) -> char {
        self.try_get(row, col)
            .expect(format!("out of bounds ({}, {})", row, col).as_str())
    }
}

impl TryFrom<char> for Section {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Section::Vertical),
            '-' => Ok(Section::Horizontal),
            '+' => Ok(Section::Cross),
            'A'..='Z' => Ok(Section::Letter(c)),
            _ => Err(()),
        }
    }
}

impl FromStr for Path {
    #[allow(unused_parens)]
    type Err = (&'static str);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.chars().filter(|c| !c.is_whitespace()).count();
        let mut path = Vec::with_capacity(len);

        let matrix = Matrix::new(
            s.lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        let start_row = 0;
        let start_col = matrix.col_where(start_row, |c| c != ' ').unwrap();
        let mut row = start_row;
        let mut col = start_col;

        assert_eq!(matrix.get(row, col), '|', "diagram doesn't begin with '|'");
        path.push(Section::Vertical);
        let mut direction = Direction::Down;

        loop {
            direction.traverse(&mut col, &mut row);
            let next: Section = if let Ok(n) = matrix.get(row, col).try_into() {
                n
            } else {
                break;
            };
            if let Section::Cross = next {
                let options: Vec<_> = vec![direction.turn_left(), direction.turn_right()]
                    .into_iter()
                    .filter(|d| {
                        let mut col = col;
                        let mut row = row;
                        d.traverse(&mut col, &mut row);
                        let cell: Section = if let Some(Ok(cell)) =
                            matrix.try_get(row, col).map(|cell| cell.try_into())
                        {
                            cell
                        } else {
                            return false;
                        };
                        match cell {
                            Section::Vertical => *d == Direction::Up || *d == Direction::Down,
                            Section::Horizontal => *d == Direction::Left || *d == Direction::Right,
                            Section::Cross => false,
                            Section::Letter(_) => true,
                        }
                    })
                    .collect();
                assert_eq!(options.len(), 1, "cannot turn: {} options", options.len());
                direction = options[0];
            }
            path.push(next);
        }

        Ok(Self(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+ ";
        let path = get_path(&input);
        println!("obtained path: {}", path);
        assert_eq!(get_path_traveled(&path), "ABCDEF");
        assert_eq!(path.0.len(), 38);
    }
}
