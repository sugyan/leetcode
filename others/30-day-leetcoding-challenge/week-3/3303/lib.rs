pub struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut m = grid;
        for i in 1..m.len() {
            m[i][0] += m[i - 1][0];
        }
        for i in 1..m[0].len() {
            m[0][i] += m[0][i - 1];
        }
        for i in 1..m.len() {
            for j in 1..m[0].len() {
                m[i][j] += std::cmp::min(m[i - 1][j], m[i][j - 1]);
            }
        }
        m[m.len() - 1][m[0].len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            7,
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        );
    }
}
