pub struct Solution {}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v: Vec<Vec<usize>> = vec![vec![]; n as usize];
        for edge in edges.iter() {
            v[edge[0] as usize].push(edge[1] as usize);
            v[edge[1] as usize].push(edge[0] as usize);
        }
        let mut longest: Vec<usize> = Vec::new();
        Solution::dfs(&v, 0, None, &mut longest);
        if longest.len() % 2 == 0 {
            vec![
                longest[longest.len() / 2 - 1] as i32,
                longest[longest.len() / 2] as i32,
            ]
        } else {
            vec![longest[longest.len() / 2] as i32]
        }
    }
    fn dfs(
        v: &[Vec<usize>],
        node: usize,
        parent: Option<usize>,
        longest: &mut Vec<usize>,
    ) -> Vec<usize> {
        let mut ret = vec![node];
        let mut children: Vec<Vec<usize>> = Vec::new();
        for i in v[node].iter() {
            if let Some(p) = parent {
                if p == *i {
                    continue;
                }
            }
            children.push(Solution::dfs(v, *i, Some(node), longest));
        }
        children.sort_by_key(|v| v.len());
        if let Some(l) = children.last() {
            ret.extend(l)
        }
        if ret.len() > longest.len() {
            *longest = ret.clone();
        }
        if children.len() > 1
            && children[children.len() - 1].len() + children[children.len() - 2].len() + 1
                > longest.len()
        {
            *longest = children[children.len() - 1]
                .iter()
                .rev()
                .chain([node].iter())
                .chain(children[children.len() - 2].iter())
                .copied()
                .collect::<Vec<usize>>();
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]);
        ret.sort();
        assert_eq!(vec![1], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::find_min_height_trees(
            6,
            vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 4]],
        );
        ret.sort();
        assert_eq!(vec![3, 4], ret);
    }
}
