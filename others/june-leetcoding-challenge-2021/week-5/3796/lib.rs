pub struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (mut i, mut flip) = (0, 0);
        for &num in &nums {
            if num == 0 {
                flip += 1;
            }
            if flip > k {
                flip -= 1 - nums[i];
                i += 1;
            }
        }
        (nums.len() - i) as i32
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
