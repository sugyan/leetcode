use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut hm: HashMap<u32, usize> = HashMap::new();
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
                } else {
                    hm.insert(k, 1);
                }
            }
        }
        hm.iter()
            .filter(|e| *e.1 > 1)
            .map(|e| {
                (0..10)
                    .map(|i| match (*e.0 >> (i * 2)) & 0b11 {
                        0b00 => 'A',
                        0b01 => 'C',
                        0b10 => 'G',
                        0b11 => 'T',
                        _ => ' ',
                    })
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret =
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string());
        ret.sort();
        assert_eq!(vec!["AAAAACCCCC", "CCCCCAAAAA"], ret);
    }
}
