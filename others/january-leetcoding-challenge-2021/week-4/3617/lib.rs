use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let (r, c) = (heights.len(), heights[0].len());
        let mut min_effort = vec![vec![None; c]; r];
        let mut bh = BinaryHeap::new();
        bh.push((Reverse(0), (0, 0)));
        while let Some((Reverse(min), (i, j))) = bh.pop() {
            if i == r - 1 && j == c - 1 {
                return min;
            }
            if min_effort[i][j].is_some() {
                continue;
            } else {
                min_effort[i][j] = Some(min);
            }
            let height = heights[i][j];
            for (i, j) in [(-1, 0), (0, -1), (1, 0), (0, 1)]
                .iter()
                .filter_map(|(di, dj)| {
                    let (ii, jj) = (i as i32 + di, j as i32 + dj);
                    if (0..r as i32).contains(&ii) && (0..c as i32).contains(&jj) {
                        Some((ii as usize, jj as usize))
                    } else {
                        None
                    }
                })
            {
                bh.push((
                    Reverse(std::cmp::max(min, (heights[i][j] - height).abs())),
                    (i, j),
                ));
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn example_1() {
        assert_eq!(
            2,
            Solution::minimum_effort_path(vec![
                vec![1, 2, 2],
                vec![3, 8, 2],
                vec![5, 3, 5]
            ])
        );
    }

    #[test]
    #[rustfmt::skip]
    fn example_2() {
        assert_eq!(
            1,
            Solution::minimum_effort_path(vec![
                vec![1, 2, 3],
                vec![3, 8, 4],
                vec![5, 3, 5]
            ])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            0,
            Solution::minimum_effort_path(vec![
                vec![1, 2, 1, 1, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 1, 1, 2, 1]
            ])
        );
    }
}
