pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut hs: HashMap<[u32; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut d = [0; 26];
            for c in s.chars() {
                d[c as usize - 'a' as usize] += 1;
            }
            if let Some(v) = hs.get_mut(&d) {
                v.push(s);
            } else {
                hs.insert(d, vec![s]);
            }
        }
        return hs.values().map(|v| v.clone()).collect::<Vec<Vec<String>>>();
    }
}

fn main() {
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .iter()
                .map(|s| s.to_string())
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
