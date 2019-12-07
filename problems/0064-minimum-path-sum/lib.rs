pub struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.to_owned();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                grid[i][j] += if i > 0 && j > 0 {
                    std::cmp::min(grid[i - 1][j], grid[i][j - 1])
                } else if i > 0 {
                    grid[i - 1][j]
                } else if j > 0 {
                    grid[i][j - 1]
                } else {
                    0
                }
            }
        }
        return *grid.last().unwrap().last().unwrap();
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
