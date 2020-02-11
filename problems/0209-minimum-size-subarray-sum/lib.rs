pub struct Solution {}

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let (mut l, mut sum) = (0, 0);
        let mut answer = nums.len();
        for i in 0..nums.len() {
            sum += nums[i];
            while sum - nums[l] >= s {
                sum -= nums[l];
                l += 1;
            }
            if sum >= s {
                answer = std::cmp::min(answer, i - l + 1);
            }
        }
        if sum < s {
            0
        } else {
            answer as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
    }
}
