use std::ops::AddAssign;
use std::str::FromStr;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input20.txt").unwrap();
    let mut particles: Vec<Particle> = input.lines().map(|l| l.parse().unwrap()).collect();
    println!(
        "particle with min acceleration is {}",
        get_min_abs_acceleration(&particles)
    );
    simulate(&mut particles, 1_000);
    println!(
        "particles not destroyed: {}",
        particles.iter().filter(|p| !p.destroyed).count()
    );
}

fn get_min_abs_acceleration(particles: &[Particle]) -> usize {
    let origin = Point::new(0, 0, 0);
    particles
        .iter()
        .enumerate()
        .min_by_key(|(_, p)| p.acceleration.manhattan_distance(&origin))
        .map(|(i, _)| i)
        .unwrap()
}

fn simulate(particles: &mut [Particle], steps: u64) {
    for _ in 0..steps {
        simulate_step(particles);
    }
}

fn simulate_step(particles: &mut [Particle]) {
    for particle in particles.iter_mut() {
        particle.next();
    }
    for destroyed in particles
        .iter()
        .enumerate()
        .filter(|(_, p)| !p.destroyed)
        .filter(|(i, p)| {
            particles
                .iter()
                .enumerate()
                .filter(|(_, p)| !p.destroyed)
                .any(|(j, q)| *i != j && p.position == q.position)
        })
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
    {
        particles[destroyed].destroyed = true;
    }
}

struct Particle {
    position: Point,
    velocity: Point,
    acceleration: Point,
    destroyed: bool,
}

impl Particle {
    pub fn next(&mut self) {
        if self.destroyed {
            return;
        }
        self.velocity += self.acceleration.clone();
        self.position += self.velocity.clone();
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Point { x, y, z }
    }
    pub fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl FromStr for Point {
    type Err = (&'static str, String);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(parts: &mut core::str::Split<char>) -> Result<i64, (&'static str, String)> {
            let s = parts
                .next()
                .ok_or(("need more numbers for 3D Point", "".to_string()))?;
            s.trim()
                .parse()
                .map_err(|_| ("invalid number", s.to_string()))
        }
        let mut parts = s.split(',');
        let x = parse(&mut parts)?;
        let y = parse(&mut parts)?;
        let z = parse(&mut parts)?;
        Ok(Point::new(x, y, z))
    }
}

impl FromStr for Particle {
    type Err = (&'static str, String);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(parts: &mut core::str::Split<&str>) -> Result<Point, (&'static str, String)> {
            let s = parts
                .next()
                .ok_or(("missing component for Particle", "".to_string()))?;
            s.trim()
                .trim_end_matches('>')
                .split("=<")
                .nth(1)
                .ok_or(("component should start with '=<'", s.to_string()))?
                .parse()
        }
        let mut parts = s.split(">,");
        let position = parse(&mut parts)?;
        let velocity = parse(&mut parts)?;
        let acceleration = parse(&mut parts)?;
        Ok(Particle {
            position,
            velocity,
            acceleration,
            destroyed: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";
        let particles: Vec<Particle> = input.lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(get_min_abs_acceleration(&particles), 0);
    }

    #[test]
    fn test2() {
        let input = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";
        let mut particles: Vec<Particle> = input.lines().map(|l| l.parse().unwrap()).collect();
        simulate(&mut particles, 100);
        assert_eq!(particles.iter().filter(|p| !p.destroyed).count(), 1);
    }
}
