use std::collections::HashMap;

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
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let mut sieve = (0..100_001).collect::<Vec<_>>();
        for i in (2..).take_while(|&i| i * i < 100_001) {
            if sieve[i] == i as i32 {
                for j in (i..100_001).step_by(i) {
                    sieve[j] = sieve[j].min(i as i32);
                }
            }
        }
        let mut hm = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let mut n = num;
            while n > 1 {
                let p = sieve[n as usize];
                hm.entry(p).or_insert_with(Vec::new).push(i);
                n /= p as i32;
            }
        }
        let mut uf = UnionFind::new(nums.len());
        for v in hm.values_mut() {
            v.dedup();
            v.windows(2).for_each(|w| uf.union(w[0], w[1]));
        }
        let mut counts = vec![0; nums.len()];
        for i in 0..nums.len() {
            counts[uf.find(i)] += 1;
        }
        *counts.iter().max().unwrap() as i32
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
