pub struct Solution;

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let (mut lo, mut hi) = (-1, -1);
        let mut answer = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num > right {
                lo = i as isize;
            }
            if num >= left {
                hi = i as isize;
            }
            answer += hi - lo;
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
            3,
            Solution::num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3)
        );
    }
}
