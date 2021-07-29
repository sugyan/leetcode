use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (r, c) = (mat.len(), mat[0].len());
        let mut answer = vec![vec![0; c]; r];
        let mut seen = vec![vec![false; c]; r];
        let mut vd = VecDeque::new();
        for (i, row) in mat.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 0 {
                    seen[i][j] = true;
                    vd.push_back(((i, j), 0));
                }
            }
        }
        while let Some(((i, j), dist)) = vd.pop_front() {
            answer[i][j] = dist;
            for d in [0, 1, 0, !0, 0].windows(2) {
                let i = i.wrapping_add(d[0]);
                let j = j.wrapping_add(d[1]);
                if i < r && j < c && !seen[i][j] {
                    seen[i][j] = true;
                    vd.push_back(((i, j), dist + 1));
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
