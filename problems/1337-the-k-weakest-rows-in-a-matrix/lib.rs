use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut bh = BinaryHeap::new();
        for (i, row) in mat.iter().enumerate() {
            bh.push(Reverse((row.iter().sum::<i32>(), i as i32)));
        }
        (0..k)
            .filter_map(|_| bh.pop().map(|Reverse((_, i))| i))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![2, 0, 3],
            Solution::k_weakest_rows(
                vec![
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1]
                ],
                3
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![0, 2],
            Solution::k_weakest_rows(
                vec![
                    vec![1, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![1, 0, 0, 0],
                    vec![1, 0, 0, 0]
                ],
                2
            )
        );
    }
}
