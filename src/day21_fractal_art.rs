use std::collections::HashMap;
use std::str::FromStr;

use log::debug;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input21.txt").unwrap();
    let mut art = generate_art(&input);
    art.steps(5);
    println!("pixels on after 5 rounds: {}", art.pixels_on());
    art.steps(13);
    println!("pixels on after 18 rounds: {}", art.pixels_on());
}

fn generate_art(input: &str) -> Art {
    Art::new(
        ".#./..#/###".parse().unwrap(),
        input.lines().map(|line| {
            line.split(" => ")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<Pattern>>()
                .try_into()
                .unwrap()
        }),
    )
}

struct Art {
    pattern: Pattern,
    rules: HashMap<Pattern, Pattern>,
}

impl Art {
    pub fn new(pattern: Pattern, rules: impl Iterator<Item = [Pattern; 2]>) -> Self {
        let mut result = HashMap::new();
        for [mut input, output] in rules {
            result.insert(input.flip_vertical(), output.clone());
            result.insert(input.clone(), output.clone());
            for _ in 0..3 {
                input = input.rotate_clockwise();
                result.insert(input.flip_vertical(), output.clone());
                result.insert(input.clone(), output.clone());
            }
        }
        Art {
            pattern,
            rules: result,
        }
    }
    pub fn pixels_on(&self) -> usize {
        self.pattern
            .0
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum()
    }
    pub fn steps(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }
    fn step(&mut self) {
        let size = self.pattern.0.len();
        let (k, new_size) = if size % 2 == 0 {
            (2, size * 3 / 2)
        } else {
            (3, size * 4 / 3)
        };
        let mut new_pattern = Pattern(vec![vec![false; new_size]; new_size]);
        for i in 0..(size / k).pow(2) {
            let grid = Grid::new(&self.pattern, i);
            grid.write_next(&mut new_pattern, &self.rules);
        }
        debug!("{:?} => {:?}", self.pattern, new_pattern);
        self.pattern = new_pattern;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pattern(Vec<Vec<bool>>);

impl Pattern {
    pub fn rotate_clockwise(&self) -> Self {
        let n = self.0.len();
        let mut result = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..n {
                result[j][n - 1 - i] = self.0[i][j];
            }
        }
        Pattern(result)
    }
    pub fn flip_vertical(&self) -> Self {
        Pattern(self.0.iter().rev().cloned().collect())
    }
}

struct Grid<'a> {
    source: &'a Pattern,
    row: usize,
    col: usize,
}

impl<'a> Grid<'a> {
    pub fn new(source: &'a Pattern, i: usize) -> Self {
        let size = source.0.len();
        let k = if size % 2 == 0 { 2 } else { 3 };
        let num_grids = size / k;
        let row = i / num_grids;
        let col = i % num_grids;
        Grid { source, row, col }
    }
    pub fn write_next(&self, target: &mut Pattern, rules: &HashMap<Pattern, Pattern>) {
        let size_source = self.source.0.len();
        let (k_source, k_target) = if size_source % 2 == 0 { (2, 3) } else { (3, 4) };

        let pattern = Pattern(
            (self.row * k_source..k_source * (self.row + 1))
                .map(|i| {
                    self.source.0[i]
                        .iter()
                        .skip(self.col * k_source)
                        .take(k_source)
                        .copied()
                        .collect()
                })
                .collect(),
        );
        let new_pattern = if let Some(p) = rules.get(&pattern) {
            p
        } else {
            panic!("no rule found for pattern {:?}", pattern);
        };
        let mut i = 0;
        for row in self.row * k_target..k_target * (self.row + 1) {
            target.0[row][self.col * k_target..k_target * (self.col + 1)]
                .copy_from_slice(&new_pattern.0[i]);
            i += 1;
        }
    }
}

impl FromStr for Pattern {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pattern(
            s.split('/')
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        env_logger::init();
        let input = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        let mut art = generate_art(input);
        art.steps(2);
        assert_eq!(art.pixels_on(), 12);
    }
}
