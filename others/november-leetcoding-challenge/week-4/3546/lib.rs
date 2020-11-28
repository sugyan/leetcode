use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut vd: VecDeque<usize> = VecDeque::new();
        let mut answer = Vec::with_capacity(nums.len() - k as usize + 1);
        for i in 0..nums.len() {
            if let Some(&front) = vd.front() {
                if front + k as usize <= i {
                    vd.pop_front();
                }
            }
            while let Some(&back) = vd.back() {
                if nums[back] < nums[i] {
                    vd.pop_back();
                } else {
                    break;
                }
            }
            vd.push_back(i);
            if i >= k as usize - 1 {
                if let Some(&front) = vd.front() {
                    answer.push(nums[front]);
                }
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
        assert_eq!(
            vec![3, 3, 5, 5, 6, 7],
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1], Solution::max_sliding_window(vec![1], 1));
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![1, -1], Solution::max_sliding_window(vec![1, -1], 1));
    }

    #[test]
    fn example_4() {
        assert_eq!(vec![11], Solution::max_sliding_window(vec![9, 11], 2));
    }

    #[test]
    fn example_5() {
        assert_eq!(vec![4], Solution::max_sliding_window(vec![4, -2], 2));
    }
}
