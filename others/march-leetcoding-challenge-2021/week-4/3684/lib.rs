use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if matrix.is_empty() {
            return Vec::new();
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut reachable = vec![vec![(false, false); n]; m];
        {
            let mut vd = VecDeque::new();
            vd.push_back((0, 0));
            (1..m).for_each(|i| vd.push_back((i, 0)));
            (1..n).for_each(|j| vd.push_back((0, j)));
            while let Some((i, j)) = vd.pop_front() {
                if reachable[i][j].0 {
                    continue;
                }
                reachable[i][j].0 = true;
                if i > 0 && matrix[i - 1][j] >= matrix[i][j] {
                    vd.push_back((i - 1, j));
                }
                if j > 0 && matrix[i][j - 1] >= matrix[i][j] {
                    vd.push_back((i, j - 1));
                }
                if i < m - 1 && matrix[i + 1][j] >= matrix[i][j] {
                    vd.push_back((i + 1, j));
                }
                if j < n - 1 && matrix[i][j + 1] >= matrix[i][j] {
                    vd.push_back((i, j + 1));
                }
            }
        }
        {
            let mut vd = VecDeque::new();
            vd.push_back((m - 1, n - 1));
            (0..m - 1).for_each(|i| vd.push_back((i, n - 1)));
            (0..n - 1).for_each(|j| vd.push_back((m - 1, j)));
            while let Some((i, j)) = vd.pop_front() {
                if reachable[i][j].1 {
                    continue;
                }
                reachable[i][j].1 = true;
                if i > 0 && matrix[i - 1][j] >= matrix[i][j] {
                    vd.push_back((i - 1, j));
                }
                if j > 0 && matrix[i][j - 1] >= matrix[i][j] {
                    vd.push_back((i, j - 1));
                }
                if i < m - 1 && matrix[i + 1][j] >= matrix[i][j] {
                    vd.push_back((i + 1, j));
                }
                if j < n - 1 && matrix[i][j + 1] >= matrix[i][j] {
                    vd.push_back((i, j + 1));
                }
            }
        }
        let mut answer = Vec::new();
        for (i, row) in reachable.iter().enumerate() {
            for (j, &(pacific, atlantic)) in row.iter().enumerate() {
                if pacific && atlantic {
                    answer.push([i as i32, j as i32].to_vec());
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
        assert_eq!(
            vec![
                vec![0, 4],
                vec![1, 3],
                vec![1, 4],
                vec![2, 2],
                vec![3, 0],
                vec![3, 1],
                vec![4, 0]
            ],
            Solution::pacific_atlantic(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4]
            ])
        );
    }
}
