use std::collections::HashMap;

const P: i64 = 31;
const MOD: i64 = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        // prepare
        let mut v = vec![0; s.len() + 1];
        let mut inv = vec![1; s.len() + 1];
        for (i, u) in s.bytes().enumerate() {
            inv[i + 1] = inv[i] * P % MOD;
            v[i + 1] = (v[i] + inv[i] * u as i64) % MOD;
        }
        inv[s.len()] = Self::pow(inv[s.len()], MOD - 2);
        for i in (0..s.len()).rev() {
            inv[i] = inv[i + 1] * P % MOD;
        }
        // binary search
        let mut answer = 0;
        let (mut lo, mut hi) = (0, s.len());
        while lo < hi {
            let mid = (lo + hi) / 2;
            if let Some(i) = Self::search(&s, &v, &inv, mid) {
                answer = i;
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        s[answer..answer + lo - 1].to_string()
    }
    fn search(s: &str, v: &[i64], inv: &[i64], len: usize) -> Option<usize> {
        let mut hm = HashMap::new();
        for i in 0..=v.len() - 1 - len {
            let hash = ((v[i + len] - v[i]).rem_euclid(MOD) * inv[i]) % MOD;
            if let Some(&j) = hm.get(&hash) {
                if s[i..i + len] == s[j..j + len] {
                    return Some(i);
                }
            }
            hm.insert(hash, i);
        }
        None
    }
    fn pow(x: i64, y: i64) -> i64 {
        let mut ret = 1;
        let mut x = x;
        let mut y = y;
        while y > 0 {
            if y & 1 > 0 {
                ret = ret * x % MOD;
            }
            x = x * x % MOD;
            y >>= 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "ana",
            Solution::longest_dup_substring(String::from("banana"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!("", Solution::longest_dup_substring(String::from("abcd")));
    }
}
