pub struct Solution;

impl Solution {
    pub fn original_digits(s: String) -> String {
        let char_counts = |s: &&str| -> [usize; 26] {
            s.as_bytes().iter().fold([0; 26], |mut acc, x| {
                acc[(x - b'a') as usize] += 1;
                acc
            })
        };
        let mut counts = char_counts(&s.as_str());
        let numbers = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .map(char_counts)
        .collect::<Vec<_>>();
        let mut answer = Vec::new();
        for &(i, b) in &[
            (0, b'z'),
            (2, b'w'),
            (4, b'u'),
            (5, b'f'),
            (6, b'x'),
            (7, b'v'),
            (8, b'g'),
            (9, b'i'),
            (1, b'n'),
            (3, b't'),
        ] {
            let n = counts[(b - b'a') as usize];
            (0..n).for_each(|_| answer.push((b'0' + i) as char));
            (0..26).for_each(|j| counts[j] -= n * numbers[i as usize][j]);
        }
        answer.sort_unstable();
        answer.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("012", Solution::original_digits(String::from("owoztneoer")));
    }

    #[test]
    fn example_2() {
        assert_eq!("45", Solution::original_digits(String::from("fviefuro")));
    }
}
