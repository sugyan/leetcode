pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut answer = 0;
        let (mut current, mut farthest) = (0, nums[0] as usize);
        while current < nums.len() - 1 {
            answer += 1;
            let range = current..=farthest.min(nums.len() - 1);
            current = farthest;
            for i in range {
                farthest = farthest.max(i + nums[i] as usize);
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
