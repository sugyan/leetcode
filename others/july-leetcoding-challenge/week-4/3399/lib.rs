pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |acc, x| acc ^ x);
        let mask = xor & -xor;
        let mut answer = vec![0, 0];
        for &num in nums.iter() {
            if num & mask == 0 {
                answer[0] ^= num;
            } else {
                answer[1] ^= num;
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
