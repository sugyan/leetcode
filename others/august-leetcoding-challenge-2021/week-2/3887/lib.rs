use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::new();
        for s in strs {
            let key = s.as_bytes().iter().fold([0; 26], |mut acc, x| {
                acc[(x - b'a') as usize] += 1;
                acc
            });
            hm.entry(key).or_insert_with(Vec::new).push(s);
        }
        hm.into_iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        for v in ret.iter_mut() {
            v.sort_unstable();
        }
        ret.sort();
        assert_eq!(
            vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"]],
            ret
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![""]],
            Solution::group_anagrams(vec![String::from("")])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec![vec!["a"]],
            Solution::group_anagrams(vec![String::from("a")])
        );
    }
}
