pub struct Solution;

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ids = vec![vec![None; n]; n];
        let mut sizes = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 && ids[i][j].is_none() {
                    let size = Self::dfs(&grid, &mut ids, (i, j), sizes.len());
                    sizes.push(size);
                }
            }
        }
        let mut answer = *sizes.iter().max().unwrap_or(&0);
        for (i, row) in grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let mut v = Vec::new();
                if *col == 0 {
                    for d in [0, 1, 0, !0, 0].windows(2) {
                        let i = i.wrapping_add(d[0]);
                        let j = j.wrapping_add(d[1]);
                        if i < n && j < n {
                            if let Some(idx) = ids[i][j] {
                                v.push(idx);
                            }
                        }
                    }
                }
                v.sort_unstable();
                v.dedup();
                answer = answer.max(1 + v.iter().map(|&k| sizes[k]).sum::<i32>());
            }
        }
        answer
    }
    fn dfs(
        grid: &[Vec<i32>],
        ids: &mut Vec<Vec<Option<usize>>>,
        p: (usize, usize),
        idx: usize,
    ) -> i32 {
        ids[p.0][p.1] = Some(idx);
        let mut ret = 1;
        for d in [0, 1, 0, !0, 0].windows(2) {
            let i = p.0.wrapping_add(d[0]);
            let j = p.1.wrapping_add(d[1]);
            if i < grid.len() && j < grid.len() && grid[i][j] == 1 && ids[i][j].is_none() {
                ret += Self::dfs(grid, ids, (i, j), idx);
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
        assert_eq!(3, Solution::largest_island(vec![vec![1, 0], vec![0, 1]]));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::largest_island(vec![vec![1, 1], vec![1, 0]]));
    }

    #[test]
    fn example_3() {
        assert_eq!(4, Solution::largest_island(vec![vec![1, 1], vec![1, 1]]));
    }
}
