use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut hm: HashMap<u32, usize> = HashMap::new();
        let mut answer: Vec<String> = Vec::new();
        if s.len() > 10 {
            let mut k = 0;
            for (i, c) in (0..).zip(s.chars()) {
                k >>= 2;
                k += match c {
                    'A' => 0b00,
                    'C' => 0b01,
                    'G' => 0b10,
                    'T' => 0b11,
                    _ => std::u32::MAX,
                } << 18;
                if i < 9 {
                    continue;
                }
                if let Some(n) = hm.get_mut(&k) {
                    *n += 1;
                    if *n == 2 {
                        answer.push((&s[i - 9..=i]).to_string());
                    }
                } else {
                    hm.insert(k, 1);
                }
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
        assert_eq!(
            vec!["AAAAACCCCC", "CCCCCAAAAA"],
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string())
        );
    }
}
