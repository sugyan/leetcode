use std::collections::HashMap;

struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: vec![-1; n + 1],
        }
    }
    fn find(&self, x: i32) -> i32 {
        let mut x = x;
        while self.parent[x as usize] != -1 {
            x = self.parent[x as usize]
        }
        x
    }
    fn union(&mut self, x: i32, y: i32) {
        let x = self.find(x);
        let y = self.find(y);
        if x != y {
            self.parent[y as usize] = x;
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn largest_component_size(a: Vec<i32>) -> i32 {
        let mut uf = UnionFind::new(*a.iter().max().unwrap() as usize);
        for &n in a.iter() {
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 {
                    uf.union(i, n);
                    uf.union(n, n / i);
                }
                i += 1;
            }
        }
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for &n in a.iter() {
            *hm.entry(uf.find(n)).or_insert(0) += 1;
        }
        hm.iter().map(|e| *e.1).max().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::largest_component_size(vec![4, 6, 15, 35]));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::largest_component_size(vec![20, 50, 9, 63]));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            8,
            Solution::largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39])
        );
    }
}
