pub struct Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut v = [1; 5];
        for _ in 1..n {
            for i in 1..5 {
                v[i] += v[i - 1];
            }
        }
        v.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::count_vowel_strings(1));
    }

    #[test]
    fn example_2() {
        assert_eq!(15, Solution::count_vowel_strings(2));
    }

    #[test]
    fn example_3() {
        assert_eq!(66045, Solution::count_vowel_strings(33));
    }
}
