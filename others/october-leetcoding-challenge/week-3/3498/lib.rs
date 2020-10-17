use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut hm: HashMap<&str, usize> = HashMap::new();
        if s.len() < 10 {
            return Vec::new();
        }
        for i in 0..s.len() - 9 {
            *hm.entry(&s[i..i + 10]).or_insert(0) += 1;
        }
        hm.iter()
            .filter(|&x| *x.1 > 1)
            .map(|x| x.0.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret =
            Solution::find_repeated_dna_sequences(String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"));
        ret.sort();
        assert_eq!(vec!["AAAAACCCCC", "CCCCCAAAAA"], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::find_repeated_dna_sequences(String::from("AAAAAAAAAAAAA"));
        ret.sort();
        assert_eq!(vec!["AAAAAAAAAA"], ret);
    }
}
