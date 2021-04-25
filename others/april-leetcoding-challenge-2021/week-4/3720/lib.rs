pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let (r, c) = (matrix.len(), matrix[0].len());
        for i in 0..r {
            for j in i..c - i - 1 {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[r - 1 - j][i];
                matrix[r - 1 - j][i] = matrix[c - 1 - i][r - 1 - j];
                matrix[c - 1 - i][r - 1 - j] = matrix[j][c - 1 - i];
                matrix[j][c - 1 - i] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn example_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3]
        ], matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn example_2() {
        let mut matrix = vec![
            vec![ 5,  1,  9, 11],
            vec![ 2,  4,  8, 10],
            vec![13,  3,  6,  7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(vec![
            vec![15, 13,  2,  5],
            vec![14,  3,  4,  1],
            vec![12,  6,  8,  9],
            vec![16,  7, 10, 11]
        ], matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn example_3() {
        let mut matrix = vec![
            vec![1]
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(vec![
            vec![1]
        ], matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn example_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4]
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(vec![
            vec![3, 1],
            vec![4, 2]
        ], matrix);
    }
}
