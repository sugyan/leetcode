pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut r = vec![false; matrix.len()];
        let mut c = vec![false; matrix[0].len()];
        for (i, row) in matrix.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 0 {
                    r[i] = true;
                    c[j] = true;
                }
            }
        }
        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if r[i] || c[j] {
                    *col = 0;
                }
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
