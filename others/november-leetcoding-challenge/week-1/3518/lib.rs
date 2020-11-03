pub struct Solution {}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut answer = 1;
        let mut power = 1;
        let mut prev = 0u8;
        for &b in s.as_bytes() {
            if b == prev {
                power += 1;
                answer = std::cmp::max(answer, power);
            } else {
                prev = b;
                power = 1;
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
        assert_eq!(2, Solution::max_power(String::from("leetcode")));
    }

    #[test]
    fn example_2() {
        assert_eq!(5, Solution::max_power(String::from("abbcccddddeeeeedcba")));
    }

    #[test]
    fn example_3() {
        assert_eq!(5, Solution::max_power(String::from("triplepillooooow")));
    }

    #[test]
    fn example_4() {
        assert_eq!(11, Solution::max_power(String::from("hooraaaaaaaaaaay")));
    }

    #[test]
    fn example_5() {
        assert_eq!(1, Solution::max_power(String::from("tourist")));
    }
}
