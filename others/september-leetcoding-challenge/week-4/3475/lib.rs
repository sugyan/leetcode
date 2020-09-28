pub struct Solution {}

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        let mut answer = 0;
        let mut p = 1;
        let mut j = 0;
        for (i, num) in nums.iter().enumerate() {
            p *= num;
            while p >= k {
                p /= nums[j];
                j += 1;
            }
            answer += i + 1 - j;
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
            8,
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100)
        );
    }
}
