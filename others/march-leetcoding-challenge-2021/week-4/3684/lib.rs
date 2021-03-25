pub struct Solution;

impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if matrix.is_empty() {
            return Vec::new();
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut p_reacheable = vec![vec![false; n]; m];
        let mut a_reacheable = vec![vec![false; n]; m];
        for i in 0..m {
            Self::dfs(&mut p_reacheable, &matrix, (i, 0));
            Self::dfs(&mut a_reacheable, &matrix, (i, n - 1));
        }
        for j in 0..n {
            Self::dfs(&mut p_reacheable, &matrix, (0, j));
            Self::dfs(&mut a_reacheable, &matrix, (m - 1, j));
        }
        let mut answer = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if p_reacheable[i][j] && a_reacheable[i][j] {
                    answer.push([i as i32, j as i32].to_vec());
                }
            }
        }
        answer
    }
    fn dfs(reachable: &mut Vec<Vec<bool>>, matrix: &[Vec<i32>], (i, j): (usize, usize)) {
        reachable[i][j] = true;
        for &(di, dj) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if (0..matrix.len() as i32).contains(&ni)
                && (0..matrix[0].len() as i32).contains(&nj)
                && !reachable[ni as usize][nj as usize]
                && matrix[ni as usize][nj as usize] >= matrix[i][j]
            {
                Self::dfs(reachable, matrix, (ni as usize, nj as usize));
            }
        }
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
