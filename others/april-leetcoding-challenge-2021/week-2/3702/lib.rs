pub struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let order = order
            .as_bytes()
            .iter()
            .enumerate()
            .fold([0; 26], |mut acc, (i, b)| {
                acc[(b - b'a') as usize] = i;
                acc
            });
        words
            .iter()
            .map(|s| {
                s.as_bytes()
                    .iter()
                    .map(|&b| order[(b - b'a') as usize])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .windows(2)
            .all(|words| words[0] <= words[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::is_alien_sorted(
                vec!["hello", "leetcode"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                String::from("hlabcdefgijkmnopqrstuvwxyz")
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::is_alien_sorted(
                vec!["word", "world", "row"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                String::from("worldabcefghijkmnpqstuvxyz")
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::is_alien_sorted(
                vec!["apple", "app"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                String::from("abcdefghijklmnopqrstuvwxyz")
            )
        );
    }
}
