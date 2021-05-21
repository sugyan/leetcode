pub struct Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let to_pattern = |s: &str| {
            let mut ret = Vec::with_capacity(s.len());
            let mut d = [None; 26];
            for (i, b) in s.as_bytes().iter().enumerate() {
                ret.push(if let Some(j) = d[(b - b'a') as usize] {
                    j
                } else {
                    d[(b - b'a') as usize] = Some(i);
                    i
                });
            }
            ret
        };
        let target = to_pattern(&pattern);
        words
            .into_iter()
            .filter(|word| to_pattern(word) == target)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec!["mee", "aqq"],
            Solution::find_and_replace_pattern(
                vec!["abc", "deq", "mee", "aqq", "dkd", "ccc"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                String::from("abb")
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec!["a", "b", "c"],
            Solution::find_and_replace_pattern(
                vec!["a", "b", "c"].into_iter().map(String::from).collect(),
                String::from("a")
            )
        );
    }
}
