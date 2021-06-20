use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let reachable = |t: i32| {
            let mut visited = vec![vec![false; n]; n];
            let mut vd = VecDeque::new();
            if grid[0][0] <= t {
                visited[0][0] = true;
                vd.push_back((0, 0));
            }
            while let Some((i, j)) = vd.pop_front() {
                if i == n - 1 && j == n - 1 {
                    return true;
                }
                for d in [0, 1, 0, !0, 0].windows(2) {
                    let i = i.wrapping_add(d[0]);
                    let j = j.wrapping_add(d[1]);
                    if (0..n).contains(&i)
                        && (0..n).contains(&j)
                        && !visited[i][j]
                        && grid[i][j] <= t
                    {
                        visited[i][j] = true;
                        vd.push_back((i, j));
                    }
                }
            }
            false
        };
        let (mut lo, mut hi) = (0, (n * n) as i32);
        while lo < hi {
            let m = (lo + hi) / 2;
            if reachable(m) {
                hi = m;
            } else {
                lo = m + 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn example_1() {
        assert_eq!(3, Solution::swim_in_water(vec![
            vec![0, 2],
            vec![1, 3]
        ]));
    }

    #[test]
    #[rustfmt::skip]
    fn example_2() {
        assert_eq!(
            16,
            Solution::swim_in_water(vec![
                vec![ 0,  1,  2,  3,  4],
                vec![24, 23, 22, 21,  5],
                vec![12, 13, 14, 15, 16],
                vec![11, 17, 18, 19, 20],
                vec![10,  9,  8,  7,  6]
            ])
        );
    }
}
