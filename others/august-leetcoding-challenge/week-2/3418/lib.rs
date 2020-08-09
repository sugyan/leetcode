use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid: Vec<Vec<i32>> = grid;
        let mut vd: VecDeque<((usize, usize), i32)> = VecDeque::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 2 {
                    vd.push_back(((i, j), 2));
                    grid[i][j] = 1;
                }
            }
        }
        while let Some(front) = vd.pop_front() {
            let p = front.0;
            if grid[p.0][p.1] != 1 {
                continue;
            }
            grid[p.0][p.1] = front.1;
            if p.0 > 0 {
                vd.push_back(((p.0 - 1, p.1), front.1 + 1));
            }
            if p.1 > 0 {
                vd.push_back(((p.0, p.1 - 1), front.1 + 1));
            }
            if p.0 < grid.len() - 1 {
                vd.push_back(((p.0 + 1, p.1), front.1 + 1));
            }
            if p.1 < grid[0].len() - 1 {
                vd.push_back(((p.0, p.1 + 1), front.1 + 1));
            }
        }
        let mut max = 2;
        for row in grid.iter() {
            for &col in row.iter() {
                if col == 1 {
                    return -1;
                }
                max = std::cmp::max(max, col);
            }
        }
        max - 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            4,
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            -1,
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::oranges_rotting(vec![vec![0, 2]]));
    }
}
