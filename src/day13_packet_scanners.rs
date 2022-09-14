use std::str::FromStr;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input13.txt").unwrap();
    let firewall: Firewall = input.parse().unwrap();
    println!("trip severity: {}", firewall.get_trip_severity());
    println!(
        "min delay to not get caught: {}",
        min_delay_to_not_get_caught(&input)
    );
}

fn min_delay_to_not_get_caught(input: &str) -> usize {
    let firewall: Firewall = input.parse().unwrap();
    for i in 0.. {
        if !firewall.would_get_caught_with_delay(i) {
            return i;
        }
    }
    panic!()
}

struct Firewall(Vec<Option<Layer>>);

impl Firewall {
    pub fn get_trip_severity(&self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(|(time, layer)| {
                layer
                    .map(|layer| {
                        if layer.is_detection_at_time(time as u32) {
                            layer.range * time as u32
                        } else {
                            0
                        }
                    })
                    .unwrap_or(0)
            })
            .sum()
    }
    pub fn would_get_caught_with_delay(&self, delay: usize) -> bool {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(time, layer)| {
                layer.map(|layer| layer.is_detection_at_time((time + delay) as u32))
            })
            .any(|x| x)
    }
}

#[derive(Clone, Copy)]
struct Layer {
    range: u32,
}

impl Layer {
    pub fn new(range: u32) -> Self {
        Self { range }
    }
    pub fn is_detection_at_time(&self, t: u32) -> bool {
        t % (2 * (self.range - 1)) == 0
    }
}

impl FromStr for Firewall {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut layers = Vec::with_capacity(
            s.lines()
                .last()
                .unwrap()
                .split(':')
                .next()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap()
                + 1usize,
        );
        for line in s.lines() {
            let mut parts = line.split(':');
            let i = parts.next().unwrap().trim().parse::<usize>().unwrap();
            let range = parts.next().unwrap().trim().parse().unwrap();
            while layers.len() < i {
                layers.push(None)
            }
            layers.push(Some(Layer::new(range)));
        }
        Ok(Self(layers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let firewall: Firewall = "0: 3
1: 2
4: 4
6: 4"
            .parse()
            .unwrap();
        assert_eq!(firewall.get_trip_severity(), 24);
    }

    #[test]
    fn test2() {
        assert_eq!(
            min_delay_to_not_get_caught(
                "0: 3
1: 2
4: 4
6: 4"
            ),
            10
        );
    }
}
