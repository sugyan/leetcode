use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let hs = nums.iter().collect::<HashSet<_>>();
        let mut answer = 0;
        for &num in &hs {
            if !hs.contains(&(num - 1)) {
                answer = answer.max((*num..).take_while(|x| hs.contains(x)).count());
            }
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            9,
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
        );
    }
}
