use std::collections::{HashMap, HashSet};

use crate::day10_knot_hash::knot_hash_bin;

pub(crate) fn run() {
    let input = "wenycdww";
    println!("used squares: {}", count_used_squares(input));
    println!("regions: {}", count_regions(input));
}

fn count_used_squares(input: &str) -> usize {
    (0..128)
        .map(|i| format!("{}-{}", input, i))
        .map(|input| {
            let hash = knot_hash_bin(&input);
            hash.chars()
                .map(|c| match c {
                    '1' => 1,
                    '0' => 0,
                    other => panic!("invalid bit '{}'", other),
                })
                .sum::<usize>()
        })
        .sum()
}

#[derive(Default)]
struct Connections {
    nodes: HashMap<u32, Connection>,
    node_id: u32,
}

#[derive(Default)]
struct Connection {
    children: HashSet<u32>,
    parent: Option<u32>,
}

impl Connections {
    pub fn new_node(&mut self) -> u32 {
        let id = self.node_id;
        self.nodes.insert(id, Default::default());
        self.node_id += 1;
        id
    }
    pub fn set_child(&mut self, parent: u32, child: u32) {
        if parent == child {
            return;
        }
        if let Some(super_parent) = self.nodes.get(&parent).unwrap().parent {
            return self.set_child(super_parent, child);
        }
        let child_node = self.nodes.get_mut(&child).unwrap();
        let grandchildren = std::mem::take(&mut child_node.children);
        for &grandchild in grandchildren.iter() {
            self.change_parent(grandchild, parent);
        }
        let parent_node = self.nodes.get_mut(&parent).unwrap();
        parent_node.children.extend(grandchildren);
        parent_node.children.insert(child);

        let mut child_node = self.nodes.get_mut(&child).unwrap();
        if let Some(existing_parent) = child_node.parent {
            return self.set_child(parent, existing_parent);
        }

        child_node.parent = Some(parent);
    }
    fn change_parent(&mut self, child: u32, new_parent: u32) {
        let mut child_node = self.nodes.get_mut(&child).unwrap();
        if child_node.parent.is_none() {
            panic!("invalid state: child {} does not have parent", child);
        }
        child_node.parent = Some(new_parent);
    }
    pub fn get_parent_nodes_count(&self) -> usize {
        self.nodes
            .values()
            .filter(|node| node.parent.is_none())
            .count()
    }
}

fn count_regions(input: &str) -> usize {
    let grid: [[bool; 128]; 128] = (0..128)
        .map(|i| format!("{}-{}", input, i))
        .map(|input| {
            let hash = knot_hash_bin(&input);
            hash.chars()
                .map(|c| match c {
                    '1' => true,
                    '0' => false,
                    other => panic!("invalid bit '{}'", other),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut connections = Connections::default();
    let mut region_map: [[Option<u32>; 128]; 128] = [[None; 128]; 128];

    if grid[0][0] {
        region_map[0][0] = Some(connections.new_node());
    }
    for i in 1..128 {
        if grid[0][i] {
            let id = if let Some(id) = region_map[0][i - 1] {
                id
            } else {
                connections.new_node()
            };
            region_map[0][i] = Some(id);
        }
    }

    for j in 1..128 {
        if grid[j][0] {
            let id = if let Some(id) = region_map[j - 1][0] {
                id
            } else {
                connections.new_node()
            };
            region_map[j][0] = Some(id);
        }
        for i in 1..128 {
            if grid[j][i] {
                let id1 = region_map[j][i - 1];
                let id2 = region_map[j - 1][i];
                let id = if let (Some(id1), Some(id2)) = (id1, id2) {
                    connections.set_child(id2, id1);
                    id2
                } else if let Some(id) = id1 {
                    id
                } else if let Some(id) = id2 {
                    id
                } else {
                    connections.new_node()
                };
                region_map[j][i] = Some(id);
            }
        }
    }

    connections.get_parent_nodes_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "flqrgnkx";
        assert_eq!(count_used_squares(input), 8108);
    }
    #[test]
    fn test2() {
        let input = "flqrgnkx";
        assert_eq!(count_regions(input), 1242);
    }
}
