pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |acc, x| acc ^ x) as i64;
        let lsb = (xor & -xor) as i32;
        let mut answer = vec![0, 0];
        for &n in &nums {
            if n & lsb == 0 {
                answer[0] ^= n;
            } else {
                answer[1] ^= n;
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
        ret.sort();
        assert_eq!(vec![3, 5], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::single_number(vec![-1, 0]);
        ret.sort();
        assert_eq!(vec![-1, 0], ret);
    }

    #[test]
    fn example_3() {
        let mut ret = Solution::single_number(vec![0, 1]);
        ret.sort();
        assert_eq!(vec![0, 1], ret);
    }
}
