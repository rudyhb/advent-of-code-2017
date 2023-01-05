use std::collections::{HashMap, HashSet};

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input7.txt").unwrap();
    let tower = build_tower(&input);
    println!("bottom program: {}", tower.name);
    let (name, weight) = balance_tower(&tower);
    println!("to balance, program '{}' should weight {}", name, weight);
}

fn build_tower(input: &str) -> Program {
    let mut results = HashMap::new();
    let mut raw_pending = HashMap::new();
    for line in input.lines() {
        let raw_program: RawProgram = line.into();
        if raw_program.children.is_empty() {
            results.insert(raw_program.name, Program::build(&raw_program, vec![]));
        } else {
            raw_pending.insert(raw_program.name, raw_program);
        }
    }
    while !raw_pending.is_empty() {
        // let removed: Vec<_> = raw_pending
        //     .drain_filter(|_, program| {
        //         program
        //             .children
        //             .iter()
        //             .all(|&child| results.contains_key(child))
        //     })
        //     .map(|(_, val)| val)
        //     .collect();
        let mut removed = Vec::new();
        for key in raw_pending
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
            .into_iter()
        {
            let program = raw_pending.get(key.as_str()).unwrap();
            if program
                .children
                .iter()
                .all(|&child| results.contains_key(child))
            {
                let program = raw_pending.remove(key.as_str()).unwrap();
                removed.push(program);
            }
        }
        if removed.is_empty() {
            panic!("could not find any more parents");
        }
        for raw_program in removed {
            let children: Vec<_> = raw_program
                .children
                .iter()
                .map(|child| results.remove(child).unwrap())
                .collect();
            results.insert(raw_program.name, Program::build(&raw_program, children));
        }
    }
    assert_eq!(results.len(), 1, "could not find root program");
    results.into_iter().next().unwrap().1
}

fn balance_tower<'a>(tower: &Program<'a>) -> (&'a str, u32) {
    let mut lower_block = tower;

    loop {
        let children_and_weights: HashMap<_, _> = lower_block
            .children
            .iter()
            .map(|child| (child.name, child.total_weight()))
            .collect();
        let weights: HashMap<u32, usize> =
            children_and_weights
                .iter()
                .fold(HashMap::new(), |mut cum, (_, &next)| {
                    *cum.entry(next).or_default() += 1;
                    cum
                });

        assert_eq!(weights.len(), 2);
        let invalid_weight = *weights
            .iter()
            .filter(|(_, &val)| val == 1)
            .next()
            .unwrap()
            .0;
        let invalid_child_name = *children_and_weights
            .iter()
            .filter(|(_, &weight)| weight == invalid_weight)
            .next()
            .unwrap()
            .0;
        lower_block = lower_block
            .children
            .iter()
            .filter(|child| child.name == invalid_child_name)
            .next()
            .unwrap();
        match lower_block
            .children
            .iter()
            .map(|child| child.total_weight())
            .collect::<HashSet<_>>()
            .len()
        {
            0 | 1 => {
                let target_weight = *weights
                    .iter()
                    .filter(|(_, &val)| val != 1)
                    .next()
                    .unwrap()
                    .0;
                return (
                    invalid_child_name,
                    target_weight
                        - lower_block
                            .children
                            .iter()
                            .map(|child| child.total_weight())
                            .sum::<u32>(),
                );
            }
            _ => {}
        }
    }
}

struct Program<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<Program<'a>>,
}

impl<'a> Program<'a> {
    pub fn build<'b>(raw: &'b RawProgram<'a>, children: Vec<Program<'a>>) -> Self {
        Self {
            name: raw.name,
            weight: raw.weight,
            children,
        }
    }
    pub fn total_weight(&self) -> u32 {
        if self.children.is_empty() {
            self.weight
        } else {
            self.weight
                + self
                    .children
                    .iter()
                    .map(|child| child.total_weight())
                    .sum::<u32>()
        }
    }
}

struct RawProgram<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<&'a str>,
}

impl<'a> From<&'a str> for RawProgram<'a> {
    fn from(s: &'a str) -> Self {
        let mut parts = s.split("->");
        let mut name_weight = parts.next().unwrap().split_whitespace();
        let name = name_weight.next().unwrap().trim();
        let parentheses: &[_] = &['(', ')'];
        let weight: u32 = name_weight
            .next()
            .unwrap()
            .trim()
            .trim_matches(parentheses)
            .parse()
            .unwrap();
        let children = if let Some(children) = parts.next() {
            children.split(',').map(|name| name.trim()).collect()
        } else {
            Vec::new()
        };
        Self {
            name,
            weight,
            children,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day07_recursive_circus::{balance_tower, build_tower};

    #[test]
    fn test1() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        let tower = build_tower(input);
        assert_eq!(tower.name, "tknk");
    }

    #[test]
    fn test2() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        let tower = build_tower(input);
        let (name, weight) = balance_tower(&tower);
        assert_eq!(name, "ugml");
        assert_eq!(weight, 60);
    }
}
