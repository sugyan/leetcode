pub struct Solution {}

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let count: usize = grid
            .iter()
            .map(|row| row.iter().filter(|&col| *col == 0).count())
            .sum();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    return Solution::helper(&mut grid, (i, j), count);
                }
            }
        }
        0
    }
    fn helper(grid: &mut Vec<Vec<i32>>, pos: (usize, usize), count: usize) -> i32 {
        if count == 0
            && ((pos.0 > 0 && grid[pos.0 - 1][pos.1] == 2)
                || (pos.1 > 0 && grid[pos.0][pos.1 - 1] == 2)
                || (pos.0 < grid.len() - 1 && grid[pos.0 + 1][pos.1] == 2)
                || (pos.1 < grid[0].len() - 1 && grid[pos.0][pos.1 + 1] == 2))
        {
            return 1;
        }
        let mut ret = 0;
        grid[pos.0][pos.1] = 1;
        if pos.0 > 0 && grid[pos.0 - 1][pos.1] == 0 {
            ret += Solution::helper(grid, (pos.0 - 1, pos.1), count - 1);
        }
        if pos.1 > 0 && grid[pos.0][pos.1 - 1] == 0 {
            ret += Solution::helper(grid, (pos.0, pos.1 - 1), count - 1);
        }
        if pos.0 < grid.len() - 1 && grid[pos.0 + 1][pos.1] == 0 {
            ret += Solution::helper(grid, (pos.0 + 1, pos.1), count - 1);
        }
        if pos.1 < grid[0].len() - 1 && grid[pos.0][pos.1 + 1] == 0 {
            ret += Solution::helper(grid, (pos.0, pos.1 + 1), count - 1);
        }
        grid[pos.0][pos.1] = 0;
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            4,
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]])
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::unique_paths_iii(vec![vec![0, 1], vec![2, 0]]));
    }
}
