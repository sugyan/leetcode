use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut hm: HashMap<&str, usize> = HashMap::new();
        if s.len() > 10 {
            for i in 0..=s.len() - 10 {
                let s = &s[i..i + 10];
                if let Some(n) = hm.get_mut(&s) {
                    *n += 1;
                } else {
                    hm.insert(s, 1);
                }
            }
        }
        hm.iter()
            .filter(|e| *e.1 > 1)
            .map(|e| (*e.0).to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec!["AAAAACCCCC", "CCCCCAAAAA"],
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string())
        );
    }
}
