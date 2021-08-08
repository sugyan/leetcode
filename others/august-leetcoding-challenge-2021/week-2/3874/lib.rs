use std::collections::BTreeMap;

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
        self.parent[y] = x
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
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (r, c) = (matrix.len(), matrix[0].len());
        let mut btm = BTreeMap::new();
        for (i, row) in matrix.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                btm.entry(col).or_insert_with(Vec::new).push((i, j));
            }
        }
        let mut rank = vec![0; r + c];
        let mut answer = vec![vec![0; c]; r];
        for v in btm.values() {
            let mut uf = UnionFind::new(r + c);
            let mut tmp_rank = rank.clone();
            for &(i, j) in v {
                let (x, y) = (uf.find(i), uf.find(j + r));
                uf.union(x, y);
                tmp_rank[uf.find(x)] = tmp_rank[x].max(tmp_rank[y]);
            }
            for &(i, j) in v {
                let new_rank = tmp_rank[uf.find(i)] + 1;
                rank[i] = new_rank;
                rank[j + r] = new_rank;
                answer[i][j] = new_rank;
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
        assert_eq!(
            vec![vec![1, 2], vec![2, 3]],
            Solution::matrix_rank_transform(vec![vec![1, 2], vec![3, 4]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![1, 1], vec![1, 1]],
            Solution::matrix_rank_transform(vec![vec![7, 7], vec![7, 7]])
        );
    }

    #[rustfmt::skip]
    #[test]
    fn example_3() {
        assert_eq!(
            vec![
                vec![4, 2, 3],
                vec![1, 3, 4],
                vec![5, 1, 6],
                vec![1, 3, 4]
            ],
            Solution::matrix_rank_transform(vec![
                vec![ 20, -21, 14],
                vec![-19,   4, 19],
                vec![ 22, -47, 24],
                vec![-19,   4, 19]
            ])
        );
    }

    #[rustfmt::skip]
    #[test]
    fn example_4() {
        assert_eq!(
            vec![
                vec![5, 1, 4],
                vec![1, 2, 3],
                vec![6, 3, 1]
            ],
            Solution::matrix_rank_transform(vec![
                vec![7, 3, 6],
                vec![1, 4, 5],
                vec![9, 8, 2]
            ])
        );
    }
}
