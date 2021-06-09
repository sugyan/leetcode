use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut bh = BinaryHeap::new();
        bh.push((nums[nums.len() - 1], nums.len() - 1));
        for i in (0..nums.len() - 1).rev() {
            while let Some(&(max, j)) = bh.peek() {
                if j - i > k as usize {
                    bh.pop();
                } else {
                    bh.push((nums[i] + max, i));
                    break;
                }
            }
        }
        while let Some(&(_, i)) = bh.peek() {
            if i != 0 {
                bh.pop();
            } else {
                break;
            }
        }
        bh.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(7, Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(17, Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            0,
            Solution::max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2)
        );
    }
}
