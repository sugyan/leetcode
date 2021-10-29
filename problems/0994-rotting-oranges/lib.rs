use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut grid = grid;
        let mut vd = VecDeque::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                if col == 2 {
                    vd.push_back((i, j));
                }
            }
        }
        while let Some((i, j)) = vd.pop_front() {
            let val = grid[i][j];
            for d in [0, 1, 0, !0, 0].windows(2) {
                let i = i.wrapping_add(d[0]);
                let j = j.wrapping_add(d[1]);
                if (0..rows).contains(&i) && (0..cols).contains(&j) && grid[i][j] == 1 {
                    grid[i][j] = val + 1;
                    vd.push_back((i, j));
                }
            }
        }
        let mut max = 2;
        for row in &grid {
            for &col in row {
                if col == 1 {
                    return -1;
                }
                max = max.max(col);
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
