pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut xor = nums
            .iter()
            .zip(1..=nums.len() as i32)
            .fold(0, |acc, (&num, i)| acc ^ num ^ i);
        xor &= -xor;
        let (mut n0, mut n1) = (0, 0);
        for &num in &nums {
            if num & xor == 0 {
                n0 ^= num;
            } else {
                n1 ^= num;
            }
        }
        for i in 1..=nums.len() as i32 {
            if i & xor == 0 {
                n0 ^= i;
            } else {
                n1 ^= i;
            }
        }
        for &num in &nums {
            if num == n0 {
                return vec![n0, n1];
            }
            if num == n1 {
                return vec![n1, n0];
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![2, 3], Solution::find_error_nums(vec![1, 2, 2, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1, 2], Solution::find_error_nums(vec![1, 1]));
    }
}
