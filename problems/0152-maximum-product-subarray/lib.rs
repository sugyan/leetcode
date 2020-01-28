pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut minmax = (nums[0], nums[0]);
        let mut answer = nums[0];
        for &n in nums[1..nums.len()].iter() {
            if n > 0 {
                minmax = (
                    std::cmp::min(minmax.0 * n, n),
                    std::cmp::max(minmax.1 * n, n),
                );
            } else {
                minmax = (
                    std::cmp::min(minmax.1 * n, n),
                    std::cmp::max(minmax.0 * n, n),
                );
            }
            answer = std::cmp::max(answer, minmax.1);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::max_product(vec![2, 3, -2, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::max_product(vec![-2, 0, -1]));
    }
}
