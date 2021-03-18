pub struct Solution;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        (1..nums.len())
            .fold((1, None), |(len, prev), i| {
                let o = (nums[i] - nums[i - 1]).cmp(&0);
                if o == std::cmp::Ordering::Equal || Some(o) == prev {
                    (len, prev)
                } else {
                    (len + 1, Some(o))
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            7,
            Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            2,
            Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        );
    }
}
