pub struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut miss = 1_i64;
        let mut i = 0;
        let mut answer = 0;
        while miss <= n as i64 {
            if i < nums.len() && nums[i] as i64 <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss <<= 1;
                answer += 1;
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
        assert_eq!(1, Solution::min_patches(vec![1, 3], 6));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::min_patches(vec![1, 5, 10], 20));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::min_patches(vec![1, 2, 2], 5));
    }
}
