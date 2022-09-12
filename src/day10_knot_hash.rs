pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input10.txt").unwrap();
    let mut knot = get_knot(256);
    execute_round(&mut knot, &parse_input(&input), &mut 0, &mut 0);
    println!("multiplying first 2 numbers: {}", knot[0] * knot[1]);

    println!("hash: {}", hash(&input));
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|i| i.trim().parse::<usize>().unwrap())
        .collect()
}

fn hash(input: &str) -> String {
    let input = convert_to_ascii_codes(&input);
    let mut knot = get_knot(256);
    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        execute_round(&mut knot, &input, &mut current_position, &mut skip_size);
    }
    let dense_hash = calculate_dense_hash(&knot, 16);
    dense_hash_to_string(&dense_hash)
}

fn get_knot(size: usize) -> Vec<usize> {
    (0..size).collect()
}

fn execute_round(
    knot: &mut Vec<usize>,
    input: &[usize],
    current_position: &mut usize,
    skip_size: &mut usize,
) {
    for &len in input.iter() {
        reverse_slice(knot, *current_position, len);
        *current_position += len + *skip_size;
        *skip_size += 1;
    }
}

fn reverse_slice(knot: &mut Vec<usize>, start: usize, len: usize) {
    let end = start + len - 1;
    for i in 0..len / 2 {
        let left = (start + i) % knot.len();
        let right = (end - i) % knot.len();
        let tmp = knot[left];
        knot[left] = knot[right];
        knot[right] = tmp;
    }
}

fn calculate_dense_hash(knot: &[usize], len: usize) -> Vec<usize> {
    (0..knot.len() / len)
        .map(|i| {
            knot[i * len..(i + 1) * len]
                .iter()
                .copied()
                .reduce(|accum, next| accum ^ next)
                .unwrap()
        })
        .collect()
}

fn convert_to_ascii_codes(input: &str) -> Vec<usize> {
    input
        .trim()
        .chars()
        .map(|c| c as u8 as usize)
        .chain(vec![17usize, 31, 73, 47, 23])
        .collect()
}

fn dense_hash_to_string(hash: &[usize]) -> String {
    hash.iter().map(|&val| format!("{:02x}", val)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut knot = get_knot(5);
        execute_round(&mut knot, &parse_input("3, 4, 1, 5"), &mut 0, &mut 0);
        println!("knot: {:?}", knot);
        assert_eq!(knot[0] * knot[1], 12);
    }

    #[test]
    fn test2() {
        assert_eq!(hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
    }
    #[test]
    fn test3() {
        assert_eq!(hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    }
    #[test]
    fn test4() {
        assert_eq!(hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    }
    #[test]
    fn test5() {
        assert_eq!(hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
