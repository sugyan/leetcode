pub struct Solution;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![Vec::new(); n as usize];
        for c in &connections {
            graph[c[0] as usize].push(c[1] as usize);
            graph[c[1] as usize].push(c[0] as usize);
        }
        let mut answer = Vec::new();
        let mut pre = vec![None; n as usize];
        let mut low = vec![None; n as usize];
        Self::dfs(&graph, 0, 0, 0, &mut answer, &mut pre, &mut low);
        answer
    }
    fn dfs(
        graph: &[Vec<usize>],
        i: usize,
        prev: usize,
        time: usize,
        answer: &mut Vec<Vec<i32>>,
        pre: &mut Vec<Option<usize>>,
        low: &mut Vec<Option<usize>>,
    ) {
        pre[i] = Some(time);
        low[i] = Some(time);
        for &j in &graph[i] {
            if pre[j].is_none() {
                Self::dfs(graph, j, i, time + 1, answer, pre, low);
                if pre[j] == low[j] {
                    answer.push([i as i32, j as i32].to_vec());
                }
            }
            if j != prev {
                if let Some(l) = low[j] {
                    low[i] = low[i].map(|m| m.min(l));
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
