pub struct Solution;

const MAPPING: [std::ops::RangeInclusive<u8>; 8] = [
    (b'a'..=b'c'),
    (b'd'..=b'f'),
    (b'g'..=b'i'),
    (b'j'..=b'l'),
    (b'm'..=b'o'),
    (b'p'..=b's'),
    (b't'..=b'v'),
    (b'w'..=b'z'),
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        digits.as_bytes().iter().fold(
            if digits.is_empty() {
                Vec::new()
            } else {
                vec![String::new()]
            },
            |acc, &x| {
                acc.iter()
                    .flat_map(|s| {
                        MAPPING[(x - b'2') as usize]
                            .clone()
                            .map(|b| s.chars().chain(std::iter::once(b as char)).collect())
                            .collect::<Vec<_>>()
                    })
                    .collect()
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            Solution::letter_combinations(String::from("23"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Vec::<String>::new(),
            Solution::letter_combinations(String::from(""))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec!["a", "b", "c"],
            Solution::letter_combinations(String::from("2"))
        );
    }
}
