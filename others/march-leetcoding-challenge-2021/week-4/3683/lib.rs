use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut bh = a.iter().map(Reverse).collect::<BinaryHeap<_>>();
        let mut b = b
            .iter()
            .enumerate()
            .map(|(i, &n)| (n, i))
            .collect::<Vec<_>>();
        b.sort_unstable();
        let mut answer = vec![-1; a.len()];
        let mut fill = Vec::new();
        for &(n, i) in &b {
            while let Some(Reverse(&m)) = bh.pop() {
                if m > n {
                    answer[i] = m;
                    break;
                }
                fill.push(m);
            }
        }
        for n in answer.iter_mut() {
            if *n == -1 {
                *n = fill.pop().unwrap();
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
        let mut ret = Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]);
        assert_eq!(
            4,
            ret.iter()
                .zip(vec![1, 10, 4, 11].iter())
                .filter(|(a, b)| a > b)
                .count()
        );
        ret.sort_unstable();
        assert_eq!(vec![2, 7, 11, 15], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]);
        assert_eq!(
            3,
            ret.iter()
                .zip(vec![13, 25, 32, 11].iter())
                .filter(|(a, b)| a > b)
                .count()
        );
        ret.sort_unstable();
        assert_eq!(vec![8, 12, 24, 32], ret);
    }
}
