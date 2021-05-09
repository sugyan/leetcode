use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut bh = BinaryHeap::new();
        let mut sum = 0;
        for n64 in target.iter().map(|&n| n as i64) {
            bh.push(n64);
            sum += n64;
        }
        while let Some(max) = bh.pop() {
            if sum <= 1 || max * 2 <= sum {
                break;
            }
            sum -= max;
            match sum.cmp(&1) {
                Ordering::Less => return false,
                Ordering::Equal => return true,
                Ordering::Greater => {}
            }
            bh.push(max % sum);
            sum += max % sum;
        }
        sum == target.len() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_possible(vec![9, 3, 5]));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_possible(vec![1, 1, 1, 2]));
    }

    #[test]
    fn example_3() {
        assert_eq!(true, Solution::is_possible(vec![8, 5]));
    }
}
