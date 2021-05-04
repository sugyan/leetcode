pub struct Solution;

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut decrease = 0;
        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                decrease += 1;
                if decrease > 1 {
                    return false;
                }
                if (2..nums.len() - 1).contains(&i)
                    && nums[i - 1] > nums[i + 1]
                    && nums[i - 2] > nums[i]
                {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::check_possibility(vec![4, 2, 3]));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::check_possibility(vec![4, 2, 1]));
    }
}
