pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut n = 0;
        for i in (0..nums.len() - 1).rev() {
            n += 1;
            if nums[i] >= n {
                n = 0;
            }
        }
        return nums.len() == 1 || n < nums[0];
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
