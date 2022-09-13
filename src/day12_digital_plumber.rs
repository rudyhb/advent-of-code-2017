use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input12.txt").unwrap();
    let connections: Connections = input.parse().unwrap();
    println!("programs in group with 0: {}", connections.get_group(0).len());
    println!("number of groups: {}", connections.get_number_of_groups());
}

struct Connections(HashMap<u32, Vec<u32>>);

impl Connections {
    pub fn get_group(&self, with_id: u32) -> HashSet<u32> {
        let mut group = HashSet::new();
        group.insert(with_id);
        let mut next = vec![with_id];
        while !next.is_empty() {
            let mut new_neighbors = vec![];
            for next in next {
                for &neighbor in self.0.get(&next).unwrap() {
                    if !group.contains(&neighbor) {
                        group.insert(neighbor);
                        new_neighbors.push(neighbor);
                    }
                }
            }
            next = new_neighbors;
        }

        group
    }
    pub fn get_number_of_groups(&self) -> usize {
        let mut already_seen = HashSet::new();
        let mut num_groups = 0;
        for i in 0..self.0.len() as u32 {
            if already_seen.contains(&i) {
                continue;
            }
            let group = self.get_group(i);
            already_seen.extend(group);
            num_groups += 1;
        }

        num_groups
    }
}

impl FromStr for Connections {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    let mut parts = line.split("<->");
                    let key = parts.next().unwrap().trim().parse().unwrap();
                    let values = parts
                        .next()
                        .unwrap()
                        .split(',')
                        .map(|val| val.trim().parse().unwrap())
                        .collect();
                    (key, values)
                })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let connections: Connections = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"
            .parse()
            .unwrap();
        let group = connections.get_group(0);
        assert_eq!(group.len(), 6);
    }

    #[test]
    fn test2() {
        let connections: Connections = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"
            .parse()
            .unwrap();
        assert_eq!(connections.get_number_of_groups(), 2);
    }
}
