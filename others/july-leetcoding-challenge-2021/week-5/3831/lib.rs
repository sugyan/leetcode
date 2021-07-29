use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (r, c) = (mat.len(), mat[0].len());
        let mut answer = vec![vec![i32::MAX; c]; r];
        let mut vd = VecDeque::new();
        for (i, row) in mat.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 0 {
                    answer[i][j] = 0;
                    vd.push_back((i, j));
                }
            }
        }
        while let Some((i, j)) = vd.pop_front() {
            for d in [0, 1, 0, !0, 0].windows(2) {
                let di = i.wrapping_add(d[0]);
                let dj = j.wrapping_add(d[1]);
                if di < r && dj < c && answer[di][dj] > answer[i][j] {
                    answer[di][dj] = answer[i][j] + 1;
                    vd.push_back((di, dj));
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
        #[rustfmt::skip]
        assert_eq!(
            vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ],
            Solution::update_matrix(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ])
        );
    }

    #[test]
    fn example_2() {
        #[rustfmt::skip]
        assert_eq!(
            vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![1, 2, 1]
            ],
            Solution::update_matrix(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![1, 1, 1]
            ])
        );
    }
}
