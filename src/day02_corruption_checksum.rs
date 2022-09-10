pub(crate) fn run() {
    println!(
        "solution: {}",
        solve_v2(&std::fs::read_to_string("input/input2.txt").unwrap())
    );
}

#[allow(unused)]
fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| row_diff(line.split_whitespace()))
        .sum()
}

#[allow(unused)]
fn solve_v2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| div_divisible(line.split_whitespace()))
        .sum()
}

#[allow(unused)]
fn row_diff<'a>(row: impl Iterator<Item = &'a str>) -> u32 {
    struct MaxMin {
        max: Option<u32>,
        min: Option<u32>,
    }
    let MaxMin { min, max } = row.fold(
        MaxMin {
            max: None,
            min: None,
        },
        |mut max_min, next| {
            let val: u32 = next.parse().unwrap();
            if max_min.min.map(|min| val < min).unwrap_or(true) {
                max_min.min = Some(val);
            }
            if max_min.max.map(|max| val > max).unwrap_or(true) {
                max_min.max = Some(val);
            }

            max_min
        },
    );

    max.unwrap() - min.unwrap()
}

#[allow(unused)]
fn div_divisible<'a>(row: impl Iterator<Item = &'a str>) -> u32 {
    let values: Vec<u32> = row.map(|val| val.parse().unwrap()).collect();
    values
        .iter()
        .enumerate()
        .flat_map(|(i, &left)| {
            values
                .iter()
                .enumerate()
                .filter(move |(j, &right)| i != *j && left % right == 0)
                .map(move |(_, &right)| left / right)
        })
        .next()
        .expect("no valid values found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                "5 1 9 5
7 5 3
2 4 6 8"
            ),
            18
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve_v2(
                "5 9 2 8
9 4 7 3
3 8 6 5"
            ),
            9
        );
    }
}
