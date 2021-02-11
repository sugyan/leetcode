pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut d = [0; 26];
        for i in s.as_bytes().iter().map(|&b| (b - b'a') as usize) {
            d[i] += 1;
        }
        for i in t.as_bytes().iter().map(|&b| (b - b'a') as usize) {
            if d[i] == 0 {
                return false;
            }
            d[i] -= 1;
        }
        d.iter().all(|&n| n == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::is_anagram(String::from("anagram"), String::from("nagaram"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::is_anagram(String::from("rat"), String::from("car"))
        );
    }
}
