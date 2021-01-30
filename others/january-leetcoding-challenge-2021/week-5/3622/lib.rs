use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut bh = nums
            .iter()
            .map(|&num| if num & 1 == 0 { num } else { num << 1 })
            .collect::<BinaryHeap<_>>();
        let mut min = *bh.iter().min().unwrap();
        let mut answer = std::i32::MAX;
        while let Some(max) = bh.pop() {
            answer = std::cmp::min(answer, max - min);
            if max & 1 == 0 {
                min = std::cmp::min(min, max >> 1);
                bh.push(max >> 1);
            } else {
                break;
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
        assert_eq!(1, Solution::minimum_deviation(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::minimum_deviation(vec![4, 1, 5, 20, 3]));
    }

    #[test]
    fn example_3() {
        assert_eq!(3, Solution::minimum_deviation(vec![2, 10, 8]));
    }
}
