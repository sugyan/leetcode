pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        for i in 0..nums.len() {
            loop {
                let num = nums[i];
                if num > 0 && num <= nums.len() as i32 && nums[i] != nums[num as usize - 1] {
                    nums.swap(i, num as usize - 1);
                } else {
                    break;
                }
            }
        }
        for (i, &num) in nums.iter().enumerate() {
            if num != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        nums.len() as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
    }
}
