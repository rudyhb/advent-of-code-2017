pub(crate) fn run() {
    let input = std::fs::read_to_string("input/input4.txt").unwrap();
    let phrases: Vec<_> = input.lines().map(|line| Passphrase::from(line)).collect();
    let valid = phrases.iter().filter(|phrase| is_valid(phrase)).count();
    println!("valid: {}", valid);
    let valid = phrases.iter().filter(|phrase| is_valid_v2(phrase)).count();
    println!("valid v2: {}", valid);
}

fn is_valid(passphrase: &Passphrase) -> bool {
    let mut words = passphrase.words.clone();
    words.sort();
    words.dedup();
    words.len() == passphrase.words.len()
}

fn is_valid_v2(passphrase: &Passphrase) -> bool {
    let mut words = passphrase.words.clone();
    words.sort();
    words.dedup();
    for i in (1..words.len()).rev() {
        let mut left: Vec<_> = words[i].chars().collect();
        left.sort();
        if (0..i).any(|j| {
            let mut right: Vec<_> = words[j].chars().collect();
            right.sort();
            left == right
        }) {
            return false;
        }
    }
    words.len() == passphrase.words.len()
}

struct Passphrase<'a> {
    words: Vec<&'a str>,
}

impl<'a> From<&'a str> for Passphrase<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            words: s.split_whitespace().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(is_valid(&Passphrase::from("aa bb cc dd ee")));
        assert!(!is_valid(&Passphrase::from("aa bb cc dd aa")));
        assert!(is_valid(&Passphrase::from("aa bb cc dd aaa")));
    }

    #[test]
    fn test2() {
        assert!(is_valid_v2(&Passphrase::from("abcde fghij")));
        assert!(!is_valid_v2(&Passphrase::from("abcde xyz ecdab")));
        assert!(is_valid_v2(&Passphrase::from("a ab abc abd abf abj")));
        assert!(is_valid_v2(&Passphrase::from("iiii oiii ooii oooi oooo")));
        assert!(!is_valid_v2(&Passphrase::from(
            "iiii oiii ooii oooi oooo oiii ioii iioi iiio"
        )));
    }
}
