use std::collections::HashMap;

use log::debug;

pub(crate) fn run() {
    // println!("answer is {}", solve_v2(120))
    println!("answer is {}", solve_v2(361527))
}

#[allow(unused)]
fn solve(i: usize) -> usize {
    struct RingData {
        ring_number: usize,
        last_number: usize,
    }
    fn get_ring_number(i: usize) -> RingData {
        let mut data = RingData {
            ring_number: 0,
            last_number: 1,
        };
        while i > data.last_number {
            data.ring_number += 1;
            data.last_number += data.ring_number * 8;
        }
        data
    }

    let ring_data = get_ring_number(i);
    if ring_data.ring_number == 0 {
        return 0;
    }
    let ring_width = (ring_data.ring_number * 8 + 4) / 4;
    let side = (1..=4)
        .filter(|&j| ring_data.last_number - j * (ring_width - 1) <= i)
        .next()
        .expect("couldn't find number in ring");
    let position = i - (ring_data.last_number - side * (ring_width - 1));
    let midpoint = ring_width / 2;
    let distance_along_ring = midpoint.abs_diff(position);
    distance_along_ring + ring_data.ring_number
}

#[allow(unused)]
fn solve_v2(limit_val: usize) -> usize {
    #[derive(Hash, Eq, PartialEq, Clone, Debug)]
    struct Point {
        x: isize,
        y: isize,
    }
    impl Point {
        pub(crate) fn up() -> Self {
            Self { x: 0, y: 1 }
        }
        pub(crate) fn down() -> Self {
            Self { x: 0, y: -1 }
        }
        pub(crate) fn left() -> Self {
            Self { x: -1, y: 0 }
        }
        pub(crate) fn right() -> Self {
            Self { x: 1, y: 0 }
        }
        pub(crate) fn add(&mut self, other: &Self) {
            self.x += other.x;
            self.y += other.y;
        }
    }
    fn get_score(data: &HashMap<Point, usize>, point: &Point) -> usize {
        (-1..=1)
            .flat_map(move |i| (-1..=1).map(move |j| (i, j)))
            .filter(|coord| *coord != (0, 0))
            .filter_map(|coord| {
                let neighbor = Point {
                    x: point.x + coord.0,
                    y: point.y + coord.1,
                };
                data.get(&neighbor)
            })
            .sum()
    }
    let mut data: HashMap<Point, usize> = HashMap::new();
    let mut point = Point { x: 0, y: 0 };
    data.insert(point.clone(), 1);
    point.x += 1;
    let mut ring_number = 0;
    let mut last_index = 1;
    let mut ring_width = 1;
    let mut direction = Point { x: 0, y: 0 };
    for i in 2.. {
        point.add(&direction);
        let score = get_score(&data, &point);
        debug!("{}: {:?} score={}", i, point, score);
        if score > limit_val {
            return score;
        }
        data.insert(point.clone(), score);
        if i == last_index + 1 {
            debug!("turning up!");
            ring_number += 1;
            ring_width = (ring_number * 8 + 4) / 4;
            last_index += ring_number * 8;
            direction = Point::up();
        } else if i == last_index - (ring_width - 1) {
            debug!("turning right!");
            direction = Point::right();
        } else if i == last_index - 2 * (ring_width - 1) {
            debug!("turning down!");
            direction = Point::down();
        } else if i == last_index - 3 * (ring_width - 1) {
            debug!("turning left!");
            direction = Point::left();
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve(1), 0)
    }
    #[test]
    fn test2() {
        assert_eq!(solve(12), 3)
    }
    #[test]
    fn test3() {
        assert_eq!(solve(23), 2)
    }
    #[test]
    fn test4() {
        assert_eq!(solve(1024), 31)
    }
}
