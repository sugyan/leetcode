use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut bh = BinaryHeap::with_capacity(k as usize);
        for (i, row) in mat.iter().enumerate() {
            let count = match row.binary_search_by(|val| 1.cmp(val)) {
                Ok(i) => i + 1,
                Err(_) => 0,
            };
            bh.push(Reverse((count, i as i32)));
        }
        (0..k)
            .filter_map(|_| bh.pop().map(|Reverse((_, i))| i))
            .collect::<Vec<_>>()
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
