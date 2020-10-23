pub struct Solution {}

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = Vec::new();
        let mut min = std::i32::MIN;
        for &num in nums.iter().rev() {
            if num < min {
                return true;
            }
            while let Some(&last) = stack.last() {
                if num > last {
                    min = stack.pop().unwrap();
                } else {
                    break;
                }
            }
            stack.push(num);
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
