use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let v: Vec<&str> = str.split(' ').collect();
        if v.len() != pattern.len() {
            return false;
        }
        let mut d1: HashMap<char, String> = HashMap::new();
        let mut d2: HashMap<String, char> = HashMap::new();
        for (i, c) in pattern.chars().enumerate() {
            if let Some(s) = d1.get(&c) {
                if s != v[i] {
                    return false;
                }
            } else if let Some(&e) = d2.get(v[i]) {
                if e != c {
                    return false;
                }
            } else {
                d1.insert(c, v[i].to_string());
                d2.insert(v[i].to_string(), c);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat dog"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat fish"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::word_pattern(String::from("aaaa"), String::from("dog cat cat dog"))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            false,
            Solution::word_pattern(String::from("abba"), String::from("dog dog dog dog"))
        );
    }
}
