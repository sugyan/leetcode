use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![0; mat[0].len()]; mat.len()];
        let (r, c) = (mat.len(), mat[0].len());
        let mut groups = vec![BinaryHeap::new(); r + c - 1];
        for (i, row) in mat.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                groups[c + i - j - 1].push(Reverse(*col));
            }
        }
        for (i, row) in ret.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if let Some(val) = groups[c + i - j - 1].pop() {
                    *col = val.0;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        #[rustfmt::skip]
        assert_eq!(
            vec![
                vec![1, 1, 1, 1],
                vec![1, 2, 2, 2],
                vec![1, 2, 3, 3]
            ],
            Solution::diagonal_sort(
                vec![
                    vec![3, 3, 1, 1],
                    vec![2, 2, 1, 2],
                    vec![1, 1, 1, 2]
                ]
            )
        );
    }
}
