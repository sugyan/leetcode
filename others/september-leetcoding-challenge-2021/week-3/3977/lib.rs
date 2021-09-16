pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (r, c) = (matrix.len(), matrix[0].len());
        let mut answer = Vec::with_capacity(r * c);
        let mut visited = vec![vec![false; c]; r];
        answer.push(matrix[0][0]);
        visited[0][0] = true;
        let (mut i, mut j) = (0_usize, 0_usize);
        for k in 0.. {
            let (di, dj) = [(0, 1), (1, 0), (0, !0), (!0, 0)][k % 4];
            loop {
                let ii = i.wrapping_add(di);
                let jj = j.wrapping_add(dj);
                if (0..r).contains(&ii) && (0..c).contains(&jj) && !visited[ii][jj] {
                    i = ii;
                    j = jj;
                    answer.push(matrix[i][j]);
                    visited[i][j] = true;
                } else {
                    break;
                }
            }
            if answer.len() == r * c {
                return answer;
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
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ])
        )
    }
}
