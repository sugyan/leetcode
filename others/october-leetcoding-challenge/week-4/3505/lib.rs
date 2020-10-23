pub struct Solution {}

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = Vec::new();
        let mut min: Vec<i32> = vec![0; nums.len()];
        min[0] = nums[0];
        for i in 1..nums.len() {
            min[i] = std::cmp::min(min[i - 1], nums[i]);
        }
        for (i, &num) in nums.iter().enumerate().rev() {
            if num > min[i] {
                while let Some(&last) = stack.last() {
                    if last <= min[i] {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                if let Some(&last) = stack.last() {
                    if last < num {
                        return true;
                    }
                }
                stack.push(num);
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
