pub struct Solution {}

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut v: Vec<(i32, i32)> = Vec::new();
        let mut min: Option<i32> = None;
        for i in 1..nums.len() {
            if let Some(m) = min {
                if nums[i] < nums[i - 1] {
                    v.push((m, nums[i - 1]));
                    min = None;
                }
            } else if nums[i] > nums[i - 1] {
                min = Some(nums[i - 1]);
            }
            for &range in v.iter() {
                if range.0 < nums[i] && nums[i] < range.1 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(false, Solution::find132pattern(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(true, Solution::find132pattern(vec![3, 1, 4, 2]));
    }

    #[test]
    fn example_3() {
        assert_eq!(true, Solution::find132pattern(vec![-1, 3, 2, 0]));
    }
}
