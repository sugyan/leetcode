use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut sum = target.iter().sum::<i32>();
        let mut bh = target.iter().cloned().collect::<BinaryHeap<_>>();
        while let Some(max) = bh.pop() {
            if max == 1 && sum == target.len() as i32 {
                return true;
            }
            if max >= sum {
                return false;
            }
            let sub = (sum - max) * (max / (sum - max) - 1).max(1);
            bh.push(max - sub);
            sum -= sub;
        }
        unreachable!()
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
