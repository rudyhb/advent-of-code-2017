use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input24.txt").unwrap();
    let components: Vec<Component> = input.lines().map(|l| l.parse().unwrap()).collect();
    let bridges = build_bridges(&components);
    println!(
        "strongest bridge has strength {}",
        bridges
            .get_strongest_bridge()
            .iter()
            .map(|c| c.strength())
            .sum::<u32>()
    );
    println!(
        "longest bridge has strength {}",
        bridges
            .get_longest_bridge()
            .iter()
            .map(|c| c.strength())
            .sum::<u32>()
    );
}

fn build_bridges(components: &[Component]) -> Bridge {
    let mut bridge = Bridge::new(0, None, None, &HashSet::new());
    bridge.build(components);
    bridge
}

#[derive(Clone)]
struct Component {
    side_a: u32,
    side_b: u32,
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.side_a, self.side_b)
    }
}

impl Component {
    pub fn strength(&self) -> u32 {
        self.side_a + self.side_b
    }
    pub fn fits_into(&self, open_port: u32) -> bool {
        self.side_a == open_port || self.side_b == open_port
    }
}

struct Bridge {
    open_port: u32,
    component: Option<Component>,
    branches: Vec<Bridge>,
    used_components: HashSet<usize>,
}

impl Display for Bridge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}{}]",
            if let Some(c) = &self.component {
                format!("{}-", c)
            } else {
                "".to_string()
            },
            self.branches
                .iter()
                .map(|b| format!("{}", b))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl Bridge {
    pub fn new(
        open_port: u32,
        component: Option<Component>,
        component_id: Option<usize>,
        used_components: &HashSet<usize>,
    ) -> Self {
        let mut used_components = used_components.clone();
        if let Some(c) = component_id {
            used_components.insert(c);
        }

        Self {
            open_port,
            component,
            branches: vec![],
            used_components,
        }
    }
    pub fn build(&mut self, components: &[Component]) {
        self.branches = components
            .iter()
            .enumerate()
            .filter(|(i, c)| c.fits_into(self.open_port) && !self.used_components.contains(i))
            .map(|(i, c)| {
                let open_port = if c.side_a == self.open_port {
                    c.side_b
                } else {
                    c.side_a
                };
                let mut bridge =
                    Bridge::new(open_port, Some(c.clone()), Some(i), &self.used_components);
                bridge.build(components);
                bridge
            })
            .collect();
    }
    pub fn get_strongest_bridge(&self) -> Vec<Component> {
        let mut max = self
            .branches
            .iter()
            .map(|b| b.get_strongest_bridge())
            .max_by_key(|b| b.iter().map(|c| c.strength()).sum::<u32>())
            .unwrap_or_default();
        if let Some(c) = &self.component {
            max.push(c.clone());
        }
        max
    }
    pub fn get_longest_bridge(&self) -> Vec<Component> {
        let mut max = self
            .branches
            .iter()
            .map(|b| b.get_longest_bridge())
            .max_by(|a, b| {
                a.len().cmp(&b.len()).then_with(|| {
                    a.iter()
                        .map(|c| c.strength())
                        .sum::<u32>()
                        .cmp(&b.iter().map(|c| c.strength()).sum::<u32>())
                })
            })
            .unwrap_or_default();
        if let Some(c) = &self.component {
            max.push(c.clone());
        }
        max
    }
}

impl FromStr for Component {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('/');
        let result = Self {
            side_a: parts.next().ok_or(())?.parse().map_err(|_e| ())?,
            side_b: parts.next().ok_or(())?.parse().map_err(|_e| ())?,
        };
        if let Some(_extra) = parts.next() {
            return Err(());
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        let components: Vec<Component> = input.lines().map(|l| l.parse().unwrap()).collect();
        let bridges = build_bridges(&components);
        assert_eq!(
            31,
            bridges
                .get_strongest_bridge()
                .iter()
                .map(|c| c.strength())
                .sum::<u32>()
        );
        assert_eq!(
            19,
            bridges
                .get_longest_bridge()
                .iter()
                .map(|c| c.strength())
                .sum::<u32>()
        );
    }
}
