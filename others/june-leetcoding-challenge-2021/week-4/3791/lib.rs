use std::cmp::Ordering;

pub struct Solution;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }
    fn union(&mut self, x: i32, y: i32) {
        let x = self.find(x);
        let y = self.find(y);
        match x.cmp(&y) {
            Ordering::Less => self.parent[y as usize] = x as usize,
            Ordering::Equal => {}
            Ordering::Greater => self.union(y, x),
        }
    }
    fn find(&mut self, x: i32) -> i32 {
        let mut x = x as usize;
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x] as usize] as usize;
            x = self.parent[x] as usize;
        }
        x as i32
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(edges.len());
        for edge in &edges {
            if uf.find(edge[0] - 1) == uf.find(edge[1] - 1) {
                return edge.clone();
            }
            uf.union(edge[0] - 1, edge[1] - 1);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![2, 3],
            Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![1, 4],
            Solution::find_redundant_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ])
        );
    }
}
