pub struct NumMatrix {
    m: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut m = matrix.clone();
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i > 0 {
                    m[i][j] += m[i - 1][j];
                }
                if j > 0 {
                    m[i][j] += m[i][j - 1];
                }
                if i > 0 && j > 0 {
                    m[i][j] -= m[i - 1][j - 1];
                }
            }
        }
        NumMatrix { m }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut answer = self.m[row2 as usize][col2 as usize];
        if row1 > 0 {
            answer -= self.m[row1 as usize - 1][col2 as usize];
        }
        if col1 > 0 {
            answer -= self.m[row2 as usize][col1 as usize - 1];
        }
        if row1 > 0 && col1 > 0 {
            answer += self.m[row1 as usize - 1][col1 as usize - 1];
        }
        answer
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

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
