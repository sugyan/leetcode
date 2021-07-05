pub struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if mat.len() * mat[0].len() != (r * c) as usize {
            return mat;
        }
        let mut answer = vec![vec![0; c as usize]; r as usize];
        let mut values = mat.iter().flat_map(|row| row.iter());
        for row in answer.iter_mut() {
            for col in row.iter_mut() {
                *col = *values.next().unwrap();
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
            vec![vec![1, 2, 3, 4]],
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 4]],
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4)
        );
    }
}
