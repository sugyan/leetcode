pub struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut counts1 = [0; 26];
        let mut counts2 = [0; 26];
        for &b in word1.as_bytes() {
            counts1[(b - b'a') as usize] += 1;
        }
        for &b in word2.as_bytes() {
            counts2[(b - b'a') as usize] += 1;
        }
        if (0..26).any(|i| (counts1[i] > 0) != (counts2[i] > 0)) {
            return false;
        }
        counts1.sort_unstable();
        counts2.sort_unstable();
        counts1 == counts2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::close_strings(String::from("abc"), String::from("bca"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::close_strings(String::from("a"), String::from("aa"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            true,
            Solution::close_strings(String::from("cabbba"), String::from("abbccc"))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            false,
            Solution::close_strings(String::from("cabbba"), String::from("aabbss"))
        );
    }
}
