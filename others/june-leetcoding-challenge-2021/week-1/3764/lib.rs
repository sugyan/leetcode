pub struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let (r, c) = (grid.len(), grid[0].len());
        let mut seen = vec![vec![false; c]; r];
        let mut answer = 0;
        for i in 0..r {
            for j in 0..c {
                if grid[i][j] == 1 && !seen[i][j] {
                    seen[i][j] = true;
                    answer = answer.max(Self::dfs(&grid, &mut seen, (i, j)));
                }
            }
        }
        answer
    }
    fn dfs(grid: &[Vec<i32>], seen: &mut Vec<Vec<bool>>, p: (usize, usize)) -> i32 {
        let mut ret = 1;
        for d in [0, 1, 0, -1, 0].windows(2) {
            let (i, j) = (p.0 as i32 + d[0], p.1 as i32 + d[1]);
            if (0..grid.len() as i32).contains(&i) && (0..grid[0].len() as i32).contains(&j) {
                let (i, j) = (i as usize, j as usize);
                if grid[i][j] == 1 && !seen[i][j] {
                    seen[i][j] = true;
                    ret += Self::dfs(grid, seen, (i, j));
                }
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
            6,
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            0,
            Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]])
        );
    }
}
