pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input9.txt").unwrap();
    let (group, garbage_count) = create_groups_and_garbage_count(&input);
    println!("total score: {}", group.total_score());
    println!("garbage count: {}", garbage_count);
}

struct Group {
    score: u32,
    children: Vec<Group>,
}

impl Group {
    pub fn new(score: u32) -> Self {
        Self {
            score,
            children: Default::default(),
        }
    }
    pub fn total_score(&self) -> u32 {
        self.score
            + self
                .children
                .iter()
                .map(|child| child.total_score())
                .sum::<u32>()
    }
    pub fn add_child(&mut self, child: Self) {
        self.children.push(child)
    }
}

fn create_groups_and_garbage_count(input: &str) -> (Group, u32) {
    if input.chars().next().expect("input is empty") != '{' {
        panic!("invalid input: should start with '{{'");
    }
    let mut stack: Vec<Group> = Vec::new();
    let mut current = Group::new(1);
    let mut inside_garbage = false;
    let mut ignore_next = false;
    let mut garbage_count = 0;

    for c in input.chars().skip(1) {
        if ignore_next {
            ignore_next = false;
            continue;
        }
        if inside_garbage {
            match c {
                '>' => {
                    inside_garbage = false;
                }
                '!' => ignore_next = true,
                _ => {
                    garbage_count += 1;
                }
            }
            continue;
        }
        match c {
            '{' => {
                let score = current.score;
                stack.push(current);
                current = Group::new(score + 1);
            }
            '}' => {
                let mut parent = if let Some(parent) = stack.pop() {
                    parent
                } else {
                    return (current, garbage_count);
                };
                parent.add_child(current);
                current = parent;
            }
            '<' => {
                inside_garbage = true;
            }
            _ => {}
        }
    }
    panic!("invalid input: group not closed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "{}";
        assert_eq!(create_groups_and_garbage_count(input).0.total_score(), 1);
    }
    #[test]
    fn test2() {
        let input = "{{{}}}";
        assert_eq!(create_groups_and_garbage_count(input).0.total_score(), 6);
    }
    #[test]
    fn test3() {
        let input = "{{},{}}";
        assert_eq!(create_groups_and_garbage_count(input).0.total_score(), 5);
    }
    #[test]
    fn test4() {
        let input = "{{{},{},{{}}}}";
        assert_eq!(create_groups_and_garbage_count(input).0.total_score(), 16);
    }
    #[test]
    fn test5() {
        let input = "{<a>,<a>,<a>,<a>}";
        assert_eq!(create_groups_and_garbage_count(input).0.total_score(), 1);
    }
    #[test]
    fn test6() {
        let input = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
        assert_eq!(create_groups_and_garbage_count(input).0.total_score(), 9);
    }
    #[test]
    fn test7() {
        let input = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
        assert_eq!(create_groups_and_garbage_count(input).0.total_score(), 9);
    }
    #[test]
    fn test8() {
        let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
        assert_eq!(create_groups_and_garbage_count(input).0.total_score(), 3);
    }
}
