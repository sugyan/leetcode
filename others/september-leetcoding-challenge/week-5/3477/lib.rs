pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut v: Vec<bool> = vec![false; s.len() + 1];
        v[0] = true;
        for i in 0..v.len() {
            if v[i] {
                for word in word_dict.iter() {
                    if s[i..].starts_with(word) {
                        v[i + word.len()] = true;
                    }
                }
            }
        }
        v[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::word_break(
                String::from("leetcode"),
                vec![String::from("leet"), String::from("code")]
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            true,
            Solution::word_break(
                String::from("applepenapple"),
                vec![String::from("apple"), String::from("pen")]
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::word_break(
                String::from("catsandog"),
                vec![
                    String::from("cats"),
                    String::from("dog"),
                    String::from("sand"),
                    String::from("and"),
                    String::from("cat")
                ]
            )
        );
    }
}
