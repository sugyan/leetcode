pub struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (mut lo, mut hi) = (0, 0);
        let mut flip = 0;
        let mut answer = 0;
        while lo < nums.len() {
            while hi < nums.len() && (nums[hi] == 1 || flip < k) {
                if nums[hi] == 0 {
                    flip += 1;
                }
                hi += 1;
            }
            answer = answer.max(hi - lo);
            while lo < nums.len() && nums[lo] == 1 {
                lo += 1;
            }
            flip -= 1;
            lo += 1;
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            6,
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            10,
            Solution::longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            )
        );
    }
}
