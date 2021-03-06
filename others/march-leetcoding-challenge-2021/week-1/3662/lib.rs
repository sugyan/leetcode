pub struct Solution;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut v = words
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>();
        v.sort_unstable();
        let mut answer = 0;
        for (i, word) in v.iter().enumerate() {
            if i + 1 >= v.len() || !v[i + 1].starts_with(&v[i]) {
                answer += word.len() + 1;
            }
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            10,
            Solution::minimum_length_encoding(
                vec!["time", "me", "bell"]
                    .into_iter()
                    .map(str::to_string)
                    .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            2,
            Solution::minimum_length_encoding(vec![String::from("t")])
        );
    }
}
