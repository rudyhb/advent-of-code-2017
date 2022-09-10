use log::info;

pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input1.txt").unwrap();
    println!("the answer is {}", solve_v2(&input));
}

#[allow(unused)]
fn solve(input: &str) -> u32 {
    let first_digit = input.chars().next().unwrap();
    let mut sum = 0;
    let mut last = first_digit;
    for c in input.chars().skip(1) {
        if last == c {
            sum += last.to_digit(10).unwrap();
        }
        last = c;
    }
    if last == first_digit {
        sum += last.to_digit(10).unwrap();
    }
    info!("input sequence '{}' gives result {}", input, sum);
    sum
}

#[allow(unused)]
fn solve_v2(input: &str) -> u32 {
    let chars: Vec<_> = input.chars().collect();
    let mut sum = 0;
    for i in 0..chars.len() {
        let j = (chars.len() / 2 + i) % chars.len();
        if chars[i] == chars[j] {
            sum += chars[i].to_digit(10).unwrap();
        }
    }
    info!("input sequence '{}' gives result {}", input, sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve("1122"), 3);
    }
    #[test]
    fn test2() {
        assert_eq!(solve("1111"), 4);
    }
    #[test]
    fn test3() {
        assert_eq!(solve("1234"), 0);
    }
    #[test]
    fn test4() {
        assert_eq!(solve("91212129"), 9);
    }
    #[test]
    fn test_b_1() {
        assert_eq!(solve_v2("1212"), 6);
    }
    #[test]
    fn test_b_2() {
        assert_eq!(solve_v2("1221"), 0);
    }
    #[test]
    fn test_b_3() {
        assert_eq!(solve_v2("123425"), 4);
    }
    #[test]
    fn test_b_4() {
        assert_eq!(solve_v2("123123"), 12);
    }
    #[test]
    fn test_b_5() {
        assert_eq!(solve_v2("12131415"), 4);
    }
}
