use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut cache = vec![vec![None; n]; m];
        let mut graph = HashMap::new();
        for i in 0..m {
            for j in 0..n {
                for d in [0, -1, 0, 1, 0].windows(2) {
                    let (ni, nj) = (i as i32 + d[0], j as i32 + d[1]);
                    if (0..m as i32).contains(&ni) && (0..n as i32).contains(&nj) {
                        let (ni, nj) = (ni as usize, nj as usize);
                        if matrix[ni][nj] > matrix[i][j] {
                            graph.entry((i, j)).or_insert_with(Vec::new).push((ni, nj));
                        }
                    }
                }
            }
        }
        let mut answer = 0;
        for i in 0..m {
            for j in 0..n {
                answer = answer.max(Self::dfs(&graph, &mut cache, i, j));
            }
        }
        answer
    }
    fn dfs(
        graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
        cache: &mut Vec<Vec<Option<i32>>>,
        i: usize,
        j: usize,
    ) -> i32 {
        if let Some(c) = cache[i][j] {
            return c;
        }
        let mut ret = 1;
        if let Some(v) = graph.get(&(i, j)) {
            for &p in v {
                ret = ret.max(Self::dfs(graph, cache, p.0, p.1) + 1);
            }
        }
        cache[i][j] = Some(ret);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn example_1() {
        assert_eq!(
            4,
            Solution::longest_increasing_path(vec![
                vec![9, 9, 4],
                vec![6, 6, 8],
                vec![2, 1, 1]
            ])
        );
    }

    #[rustfmt::skip]
    #[test]
    fn example_2() {
        assert_eq!(
            4,
            Solution::longest_increasing_path(vec![
                vec![3, 4, 5],
                vec![3, 2, 6],
                vec![2, 2, 1]
            ])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::longest_increasing_path(vec![vec![1]]));
    }
}
