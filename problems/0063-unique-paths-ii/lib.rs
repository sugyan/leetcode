pub struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid[0].len(), obstacle_grid.len());
        let mut matrix = vec![vec![0; m]; n];
        for i in 0..n {
            if obstacle_grid[i][0] == 1 {
                break;
            }
            matrix[i][0] = 1;
        }
        for j in 0..m {
            if obstacle_grid[0][j] == 1 {
                break;
            }
            matrix[0][j] = 1;
        }
        for i in 1..n {
            for j in 1..m {
                matrix[i][j] = if obstacle_grid[i][j] == 1 {
                    0
                } else {
                    (if obstacle_grid[i - 1][j] == 1 {
                        0
                    } else {
                        matrix[i - 1][j]
                    }) + (if obstacle_grid[i][j - 1] == 1 {
                        0
                    } else {
                        matrix[i][j - 1]
                    })
                };
            }
        }
        return matrix[n - 1][m - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ])
        );
    }
}
