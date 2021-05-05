pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut target = nums.len() - 1;
        let mut answer = 0;
        while let Some(i) = (0..target).position(|i| nums[i] as usize > target - 1 - i) {
            answer += 1;
            target = i;
            if target == 0 {
                break;
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
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
    }
}
