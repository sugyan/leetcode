use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut bh = BinaryHeap::new();
        for (i, row) in matrix.iter().enumerate() {
            bh.push((Reverse(row[0]), i));
        }
        let mut idx = vec![0; matrix.len()];
        let mut k = k;
        while let Some((Reverse(val), i)) = bh.pop() {
            k -= 1;
            if k == 0 {
                return val;
            }
            idx[i] += 1;
            if idx[i] < matrix[i].len() {
                bh.push((Reverse(matrix[i][idx[i]]), i));
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            13,
            Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(-5, Solution::kth_smallest(vec![vec![-5]], 1));
    }
}
