pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut p = [-1; 128];
        let mut l = -1;
        let mut answer = 0;
        for (i, &b) in s.as_bytes().iter().enumerate() {
            l = std::cmp::max(l, p[b as usize]);
            answer = std::cmp::max(answer, i as i32 - l);
            p[b as usize] = i as i32;
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
            3,
            Solution::length_of_longest_substring(String::from("abcabcbb"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::length_of_longest_substring(String::from("bbbbb"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("pwwkew"))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(0, Solution::length_of_longest_substring(String::from("")));
    }
}
