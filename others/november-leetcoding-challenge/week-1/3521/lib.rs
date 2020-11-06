pub struct Solution {}

impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let (mut l, mut r) = (1, *nums.iter().max().unwrap());
        while l < r {
            let m = l + (r - l) / 2;
            let sum: i32 = nums
                .iter()
                .map(|&num| num / m + if num % m == 0 { 0 } else { 1 })
                .sum();
            if sum > threshold {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::smallest_divisor(vec![1, 2, 5, 9], 6));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::smallest_divisor(vec![2, 3, 5, 7, 11], 11));
    }

    #[test]
    fn example_3() {
        assert_eq!(4, Solution::smallest_divisor(vec![19], 5));
    }
}
