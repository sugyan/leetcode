pub struct Solution {}

impl Solution {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let size = a.len();
        let mut padded = vec![vec![0; size * 3 - 2]; size * 3 - 2];
        for i in 0..size {
            for j in 0..size {
                padded[i + size - 1][j + size - 1] = b[i][j];
            }
        }
        let mut answer = 0;
        for i in 0..size * 2 - 1 {
            for j in 0..size * 2 - 1 {
                answer = std::cmp::max(answer, Solution::convolution(&a, &padded, (i, j)));
            }
        }
        answer
    }
    fn convolution(a: &[Vec<i32>], p: &[Vec<i32>], shift: (usize, usize)) -> i32 {
        let mut ret = 0;
        for i in 0..a.len() {
            for j in 0..a.len() {
                ret += a[i][j] * p[i + shift.0][j + shift.1];
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
            )
        );
    }
}
