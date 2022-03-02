pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t = t.chars();
        s.chars().all(|cs| t.any(|ct| ct == cs))
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
            Solution::is_subsequence(String::from("abc"), String::from("ahbgdc"))
        );
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::is_subsequence(String::from("axc"), String::from("ahbgdc"))
        );
    }
}
