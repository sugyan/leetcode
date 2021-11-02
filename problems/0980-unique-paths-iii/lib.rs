pub struct Solution;

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut one = (0, 0);
        let mut zeros = 0;
        for (i, row) in grid.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                match *col {
                    0 => zeros += 1,
                    1 => {
                        one = (i, j);
                        *col = 0;
                    }
                    _ => {}
                }
            }
        }
        Self::backtrack(&mut grid, one, zeros + 1)
    }
    fn backtrack(grid: &mut Vec<Vec<i32>>, p: (usize, usize), zeros: usize) -> i32 {
        if grid[p.0][p.1] == 2 {
            return if zeros == 0 { 1 } else { 0 };
        }
        let mut ret = 0;
        grid[p.0][p.1] = -1;
        for d in [0, 1, 0, !0, 0].windows(2) {
            let i = p.0.wrapping_add(d[0]);
            let j = p.1.wrapping_add(d[1]);
            if (0..grid.len()).contains(&i) && (0..grid[0].len()).contains(&j) && grid[i][j] != -1 {
                ret += Self::backtrack(grid, (i, j), zeros - 1);
            }
        }
        grid[p.0][p.1] = 0;
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::unique_paths_iii(vec![
                vec![ 1,  0,  0,  0],
                vec![ 0,  0,  0,  0],
                vec![ 0,  0,  2, -1]
            ])
        );
    }

    #[rustfmt::skip]
    #[test]
    fn example_2() {
        assert_eq!(
            4,
            Solution::unique_paths_iii(vec![
                vec![ 1,  0,  0,  0],
                vec![ 0,  0,  0,  0],
                vec![ 0,  0,  0,  2]
            ])
        );
    }

    #[rustfmt::skip]
    #[test]
    fn example_3() {
        assert_eq!(
            0, Solution::unique_paths_iii(vec![
                vec![ 0,  1],
                vec![ 2,  0]
            ])
        );
    }
}
