pub struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut ab = vec![None; graph.len()];
        for i in 0..ab.len() {
            if ab[i].is_some() {
                continue;
            }
            ab[i] = Some(true);
            let mut stack = vec![(i, true)];
            while let Some((j, b)) = stack.pop() {
                for &k in &graph[j] {
                    match &mut ab[k as usize] {
                        Some(bb) if *bb == b => {
                            return false;
                        }
                        Some(_) => {}
                        None => {
                            ab[k as usize] = Some(!b);
                            stack.push((k as usize, !b));
                        }
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
