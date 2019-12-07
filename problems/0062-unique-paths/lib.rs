pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut matrix = vec![vec![1; m as usize]; n as usize];
        for i in 1..n as usize {
            for j in 1..m as usize {
                matrix[i][j] = matrix[i - 1][j] + matrix[i][j - 1];
            }
        }
        return matrix[n as usize - 1][m as usize - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::unique_paths(3, 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(28, Solution::unique_paths(7, 3));
    }
}
