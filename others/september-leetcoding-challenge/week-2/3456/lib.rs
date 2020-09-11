pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut answer = std::i32::MIN;
        let mut m = (1, 1);
        for &num in nums.iter() {
            m = if num > 0 {
                (std::cmp::min(m.0 * num, num), std::cmp::max(m.1 * num, num))
            } else {
                (std::cmp::min(m.1 * num, num), std::cmp::max(m.0 * num, num))
            };
            answer = std::cmp::max(answer, m.1);
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
