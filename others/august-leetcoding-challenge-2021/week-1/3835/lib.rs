struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }
    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        self.parent[y] = x;
    }
    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
}

pub struct Solution;

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut uf = UnionFind::new(n * n);
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    if i > 0 && grid[i - 1][j] == 1 {
                        uf.union((i - 1) * n + j, i * n + j);
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        uf.union(i * n + j - 1, i * n + j);
                    }
                }
            }
        }
        let mut sizes = vec![0; n * n];
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    sizes[uf.find(i * n + j)] += 1;
                }
            }
        }
        let mut answer = *sizes.iter().max().unwrap_or(&0);
        for i in 0..n {
            for j in 0..n {
                let mut v = Vec::new();
                if grid[i][j] == 0 {
                    for d in [0, 1, 0, !0, 0].windows(2) {
                        let i = i.wrapping_add(d[0]);
                        let j = j.wrapping_add(d[1]);
                        if i < n && j < n && grid[i][j] == 1 {
                            v.push(uf.find(i * n + j));
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
