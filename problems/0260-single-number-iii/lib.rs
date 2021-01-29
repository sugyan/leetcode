pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |acc, x| acc ^ x);
        let r = xor & !(xor - 1);
        let mut answer = vec![0, 0];
        for n in nums.iter() {
            if n & r == 0 {
                answer[0] ^= *n;
            } else {
                answer[1] ^= *n;
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
        let mut ret = Solution::single_number(vec![1, 2, 1, 3, 2, 5]);
        ret.sort_unstable();
        assert_eq!(vec![3, 5], ret);
    }
}
