use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm: HashMap<[usize; 26], usize> = HashMap::new();
        let mut answer: Vec<Vec<String>> = Vec::new();
        for s in strs.iter() {
            let mut d: [usize; 26] = [0; 26];
            for c in s.chars() {
                d[(c as u8 - b'a') as usize] += 1;
            }
            if let Some(i) = hm.get(&d) {
                answer[*i].push(s.to_string());
            } else {
                answer.push(vec![s.to_string()]);
                hm.insert(d, hm.len());
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
        let mut ret = Solution::group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        );
        for v in ret.iter_mut() {
            v.sort();
        }
        ret.sort();
        assert_eq!(
            vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"]],
            ret
        );
    }
}
