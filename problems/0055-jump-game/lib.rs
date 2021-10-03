pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut n = 1;
        for i in (0..nums.len() - 1).rev() {
            n = if nums[i] < n { n + 1 } else { 1 };
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}
