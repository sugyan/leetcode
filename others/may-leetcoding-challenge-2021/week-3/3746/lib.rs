use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words;
        words.sort_by_cached_key(String::len);
        let mut hm = HashMap::new();
        let mut answer = 0;
        for word in &words {
            let max = (0..word.len())
                .filter_map(|i| hm.get(&(String::new() + &word[0..i] + &word[i + 1..])))
                .max()
                .unwrap_or(&0)
                + 1;
            hm.insert(word, max);
            answer = answer.max(max);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            4,
            Solution::longest_str_chain(
                vec!["a", "b", "ba", "bca", "bda", "bdca"]
                    .into_iter()
                    .map(str::to_string)
                    .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            5,
            Solution::longest_str_chain(
                vec!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]
                    .into_iter()
                    .map(str::to_string)
                    .collect()
            )
        );
    }
}
