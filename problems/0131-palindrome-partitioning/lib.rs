use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut hm: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        Solution::helper(&mut hm, &s);
        hm.get(&s).unwrap().clone()
    }
    fn helper(hm: &mut HashMap<String, Vec<Vec<String>>>, s: &str) -> Vec<Vec<String>> {
        if let Some(ret) = hm.get(&s.to_string()) {
            return ret.clone();
        }
        let mut ret: Vec<Vec<String>> = Vec::new();
        if s.len() == 1 {
            hm.insert(s.to_string(), vec![vec![s.to_string()]]);
            return vec![vec![s.to_string()]];
        }
        for i in 1..=s.len() {
            let c: Vec<char> = s[0..i].chars().collect();
            if (0..i / 2).all(|j| c[j] == c[i - 1 - j]) {
                if i == s.len() {
                    ret.push(vec![s[0..i].to_string()])
                } else {
                    for a in Solution::helper(hm, &s[i..]) {
                        let mut v: Vec<String> = vec![s[0..i].to_string()];
                        v.extend(a.into_iter());
                        ret.push(v);
                    }
                }
            }
        }
        hm.insert(s.to_string(), ret.clone());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::partition("aab".to_string());
        ret.sort();
        assert_eq!(vec![vec!["a", "a", "b"], vec!["aa", "b"]], ret);
    }
}
