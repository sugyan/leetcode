use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut ab = vec![None; graph.len()];
        for i in 0..ab.len() {
            if ab[i].is_some() {
                continue;
            }
            ab[i] = Some(true);
            let mut vd = VecDeque::new();
            vd.push_back(i);
            while let Some(j) = vd.pop_front() {
                for k in graph[j].iter().map(|&k| k as usize) {
                    if ab[k] == ab[j] {
                        return false;
                    }
                    if ab[k].is_none() {
                        ab[k] = ab[j].map(|b| !b);
                        vd.push_back(k);
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]])
        );
    }
}
