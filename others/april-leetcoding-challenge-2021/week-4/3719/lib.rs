pub struct Solution;

struct Params {
    time: usize,
    visited: Vec<bool>,
    discovered: Vec<usize>,
    low: Vec<usize>,
    parent: Vec<Option<usize>>,
}

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![Vec::new(); n as usize];
        for c in &connections {
            graph[c[0] as usize].push(c[1] as usize);
            graph[c[1] as usize].push(c[0] as usize);
        }
        let mut answer = Vec::new();
        let mut params = Params {
            time: 0,
            visited: vec![false; n as usize],
            discovered: vec![0; n as usize],
            low: vec![0; n as usize],
            parent: vec![None; n as usize],
        };
        for i in 0..n as usize {
            if !params.visited[i] {
                Self::dfs(&graph, i, &mut answer, &mut params);
            }
        }
        answer
    }
    fn dfs(graph: &[Vec<usize>], i: usize, answer: &mut Vec<Vec<i32>>, params: &mut Params) {
        params.visited[i] = true;
        params.time += 1;
        params.discovered[i] = params.time;
        params.low[i] = params.time;
        for &j in &graph[i] {
            if params.visited[j] {
                if Some(j) != params.parent[i] {
                    params.low[i] = params.low[i].min(params.discovered[j]);
                }
            } else {
                params.parent[j] = Some(i);
                Self::dfs(graph, j, answer, params);
                params.low[i] = params.low[i].min(params.low[j]);
                if params.low[j] > params.discovered[i] {
                    answer.push([i as i32, j as i32].to_vec());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![vec![1, 3]],
            Solution::critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]])
        );
    }
}
