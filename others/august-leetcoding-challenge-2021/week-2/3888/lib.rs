pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (r, c) = (matrix.len(), matrix[0].len());
        let first = (0..r).any(|i| matrix[i][0] == 0);
        for i in 0..r {
            for j in 1..c {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 1..r {
            for j in 1..c {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        if matrix[0][0] == 0 {
            for j in 1..c {
                matrix[0][j] = 0;
            }
        }
        if first {
            for row in matrix.iter_mut() {
                row[0] = 0;
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
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1]
        ];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(vec![
            vec![1, 0, 1],
            vec![0, 0, 0],
            vec![1, 0, 1]
        ], matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn example_2() {
        let mut matrix = vec![
            vec![0, 1, 2, 0],
            vec![3, 4, 5, 2],
            vec![1, 3, 1, 5]
        ];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(vec![
            vec![0, 0, 0, 0],
            vec![0, 4, 5, 0],
            vec![0, 3, 1, 0]
        ], matrix);
    }
}
