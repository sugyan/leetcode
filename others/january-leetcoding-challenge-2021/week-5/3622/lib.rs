use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut bts = nums.into_iter().collect::<BTreeSet<_>>();
        let mut answer = std::i32::MAX;
        loop {
            if let (Some(&min), Some(&max)) = (bts.iter().next(), bts.iter().next_back()) {
                answer = std::cmp::min(answer, max - min);
                if min % 2 == 1 && max - min > (min * 2 - max).abs() {
                    bts.remove(&min);
                    bts.insert(min * 2);
                } else {
                    break;
                }
            }
        }
        loop {
            if let (Some(&min), Some(&max)) = (bts.iter().next(), bts.iter().next_back()) {
                answer = std::cmp::min(answer, max - min);
                if max % 2 == 0 && max - min > (max / 2 - min).abs() {
                    bts.remove(&max);
                    bts.insert(max / 2);
                } else {
                    break;
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
