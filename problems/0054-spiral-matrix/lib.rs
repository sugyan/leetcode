pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        if m == 0 {
            return vec![];
        }
        let n = matrix[0].len();
        let mut answer = vec![0; m * n];
        let mut idx = 0;
        for i in 0..(std::cmp::min(m, n) + 1) / 2 {
            let h = m - 2 * i;
            let w = n - 2 * i;
            for j in 0..w - 1 {
                answer[idx] = matrix[i][i + j];
                idx += 1;
            }
            for j in 0..h - 1 {
                answer[idx] = matrix[i + j][i + w - 1];
                idx += 1;
            }
            if h > 1 {
                for j in 0..w - 1 {
                    answer[idx] = matrix[i + h - 1][i + w - 1 - j];
                    idx += 1;
                }
            } else {
                answer[idx] = matrix[i][i + w - 1];
            }
            if w > 1 {
                for j in 0..h - 1 {
                    answer[idx] = matrix[i + h - 1 - j][i];
                    idx += 1;
                }
            } else {
                answer[idx] = matrix[i + h - 1][i];
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
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
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
        );
    }
}
