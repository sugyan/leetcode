pub struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let can_split = |n| {
            let mut count = 0;
            let mut sum = 0;
            for &num in &nums {
                if sum + num > n {
                    count += 1;
                    sum = num;
                } else {
                    sum += num;
                }
            }
            count < m
        };
        let mut lo = *nums.iter().max().unwrap();
        let mut hi = nums.iter().sum();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if can_split(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(18, Solution::split_array(vec![7, 2, 5, 10, 8], 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(9, Solution::split_array(vec![1, 2, 3, 4, 5], 2));
    }

    #[test]
    fn example_3() {
        assert_eq!(4, Solution::split_array(vec![1, 4, 4], 3));
    }
}
