pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let d1 = s1.bytes().fold([0; 26], |mut acc, u| {
            acc[(u - b'a') as usize] += 1;
            acc
        });
        let mut d2 = [0; 26];
        let s2 = s2.bytes().map(|u| (u - b'a') as usize).collect::<Vec<_>>();
        for i in 0..s2.len() {
            d2[s2[i]] += 1;
            if i >= s1.len() {
                d2[s2[i - s1.len()]] -= 1;
            }
            if d1 == d2 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::check_inclusion(String::from("ab"), String::from("eidbaooo"))
        );
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::check_inclusion(String::from("ab"), String::from("eidboaoo"))
        );
    }
}
