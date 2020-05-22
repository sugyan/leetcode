use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut hm: HashMap<u8, usize> = HashMap::with_capacity(128);
        for c in s.as_bytes() {
            *hm.entry(*c).or_insert(0) += 1;
        }
        let mut v: Vec<(&u8, &usize)> = hm.iter().collect();
        v.sort_by_cached_key(|e| e.1);
        let mut answer = String::with_capacity(s.len());
        for e in v.iter().rev() {
            for _ in 0..*e.1 {
                answer.push(*e.0 as char);
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(
            vec!["eert", "eetr"].contains(&Solution::frequency_sort("tree".to_string()).as_str())
        );
    }

    #[test]
    fn example_2() {
        assert!(vec!["cccaaa", "aaaccc"]
            .contains(&Solution::frequency_sort("cccaaa".to_string()).as_str()));
    }

    #[test]
    fn example_3() {
        assert!(
            vec!["bbAa", "bbaA"].contains(&Solution::frequency_sort("Aabb".to_string()).as_str())
        );
    }
}
