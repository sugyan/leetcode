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

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(edges.len());
        for edge in edges {
            let (e0, e1) = (edge[0] as usize - 1, edge[1] as usize - 1);
            if uf.find(e0) == uf.find(e1) {
                return edge;
            }
            uf.union(e0, e1);
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
