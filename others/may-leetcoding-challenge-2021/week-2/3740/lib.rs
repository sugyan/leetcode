pub struct NumMatrix {
    sums: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut sums = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                sums[i + 1][j + 1] = sums[i + 1][j] + sums[i][j + 1] + matrix[i][j] - sums[i][j];
            }
        }
        Self { sums }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.sums[row2 as usize + 1][col2 as usize + 1]
            - self.sums[row1 as usize][col2 as usize + 1]
            - self.sums[row2 as usize + 1][col1 as usize]
            + self.sums[row1 as usize][col1 as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let obj = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);
        assert_eq!(8, obj.sum_region(2, 1, 4, 3));
        assert_eq!(11, obj.sum_region(1, 1, 2, 2));
        assert_eq!(12, obj.sum_region(1, 2, 2, 4));
    }
}
