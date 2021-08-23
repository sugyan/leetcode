use std::collections::{HashMap, HashSet};

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut hsx = HashSet::new();
        let mut hsy = HashSet::new();
        for r in &rectangles {
            hsx.insert(r[0]);
            hsy.insert(r[1]);
            hsx.insert(r[2]);
            hsy.insert(r[3]);
        }
        let mut vx = hsx.iter().collect::<Vec<_>>();
        let mut vy = hsy.iter().collect::<Vec<_>>();
        vx.sort_unstable();
        vy.sort_unstable();
        let hmx = vx
            .iter()
            .enumerate()
            .map(|(i, &val)| (*val, i))
            .collect::<HashMap<_, _>>();
        let hmy = vy
            .iter()
            .enumerate()
            .map(|(i, &val)| (*val, i))
            .collect::<HashMap<_, _>>();
        let mut grid = vec![vec![false; vx.len()]; vy.len()];
        for r in &rectangles {
            if let (Some(&r0), Some(&r1), Some(&r2), Some(&r3)) = (
                hmx.get(&r[0]),
                hmy.get(&r[1]),
                hmx.get(&r[2]),
                hmy.get(&r[3]),
            ) {
                for x in r0..r2 {
                    for y in r1..r3 {
                        grid[y][x] = true;
                    }
                }
            }
        }
        let mut answer = 0;
        for (y, row) in grid.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                if col {
                    answer += (vx[x + 1] - vx[x]) as i64 * (vy[y + 1] - vy[y]) as i64
                }
            }
        }
        (answer % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            6,
            Solution::rectangle_area(vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            49,
            Solution::rectangle_area(vec![vec![0, 0, 1000000000, 1000000000]])
        );
    }
}
